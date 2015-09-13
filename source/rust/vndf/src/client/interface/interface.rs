use std::io::{
    self,
    stdin,
};
use std::sync::mpsc::{
    channel,
    Receiver,
    TryRecvError,
};
use std::thread::spawn;
use std::vec::Drain;

use glutin::Event;
use glutin::Event::{Closed};

use shared::physics::collision::{Collider};

use client::config::Config;
use client::console;
use client::graphics::Renderer;
use client::interface::{
    Frame,
    InputEvent,
};
use client::keyboard::Keyboard;
use client::mouse::Mouse;
use client::window::Window;

const MAX_FRAME_TIME: f64 = 0.020; // 15ms minimum frame time

pub trait Interface: Sized {
    fn new(config: Config) -> io::Result<Self>;
    fn update(&mut self, frame: &mut Frame) -> io::Result<Drain<InputEvent>>;
    fn get_config(&self) -> Option<Config>;
}


pub struct Player {
    events  : Vec<InputEvent>,
    cli     : console::Controller,
    window  : Window,
    renderer: Renderer,
    mouse   : Mouse, // NOTE: this might be renamed to selector or controller
    keyboard: Keyboard,
    config  : Config,
}

impl Interface for Player {
    fn new(config: Config) -> io::Result<Player> {
        let cli    = console::Controller::new();
        let window = Window::new(
            (800.0 * config.scaling_factor) as u32,
            (600.0 * config.scaling_factor) as u32,
        );

        let renderer = Renderer::new(&window, config.scaling_factor);

        Ok(Player {
            events  : Vec::new(),
            cli     : cli,
            window  : window,
            renderer: renderer,
            mouse   : Mouse::new(),
            keyboard: Keyboard::new(),
            config  : config,
        })
    }

    fn update(&mut self, frame: &mut Frame)
              -> io::Result<Drain<InputEvent>> {
        let window_events: Vec<Event> = self.window.poll_events().collect();

        // handle a closed-window event
        for event in window_events.iter() {
            match *event {
                Closed => self.events.push(InputEvent::Quit), 
                _ => {},
            }
        }
        
        self.keyboard.update(&mut self.events,
                             frame,
                             &window_events,
                             &mut self.renderer.camera);

        if let Some(size) = self.window.get_size().ok() {
        self.mouse.update(&mut self.events,
                          frame,
                          &window_events,
                          size,
                          &mut self.renderer.camera);
        }
        
        self.cli.update(&mut self.events, frame, &window_events);
        
        if let Some(track) = frame.camera_track.clone() {
            self.renderer.camera.set_track(track);
            frame.camera_track = None; //we should clear this out
        }

        // interpolate ship position
        for (_,ship) in frame.ships.iter_mut() {
            let pos = ship.position+(ship.velocity*frame.deltatime*1.99);
            ship.position = pos;
        }
        
        self.renderer.render(
            frame,
            &self.cli.console,
            &self.window,
        );
        self.window.swap_buffers();

	check_collisions(frame,&mut self.events, self.renderer.camera.zoom);

        // frame delay notifier
        if frame.deltatime > MAX_FRAME_TIME {
            // notify of frame delays
            // TODO: add event type to push (FrameDelay(dt:f64))
        }
        
        Ok(self.events.drain(..))
    }

    fn get_config (&self) -> Option<Config> { Some(self.config.clone()) }
}

fn check_collisions(frame: &mut Frame,
                    events: &mut Vec<InputEvent>,
                    zoom: f32) {
    // TODO: needs some notion of space-partitioning for efficiency
    'ships: for (ship_id,ship_body) in frame.ships.iter() {
	let ship_coll = {
	    if let Some (coll) = frame.colliders.get(&ship_id) { coll }
	    else { warn!("No collider found for ship {}", ship_id);
		   continue 'ships }
	};

        // check ships colliding into planets
	'planets: for (planet_id,planet) in frame.planets.iter() {
	    let planet_coll = {
		if let Some (coll) = frame.colliders.get(&planet_id) { coll }
		else { warn!("No collider found for planet {}", planet_id);
		       continue 'planets }
	    };
	    if ship_coll.check_collision(&ship_body.position,
					 (planet_coll,&planet.body.position)) {
		events.push(InputEvent::Collision(*ship_id,*planet_id));
	    }
	}

        // check ships colliding into eachother
	'other_ships: for (ship_id2,ship_body2) in frame.ships.iter() {
	    if ship_id == ship_id2 { continue 'other_ships }

            // NOTE: we need two separate checks, one for actual collisions
            // and another for visual collisions while zoomed (ie: to be grouped)
            let ship_coll2 = {
		if let Some (coll) = frame.colliders.get(&ship_id2) { coll }
		else { warn!("No collider found for ship {}", ship_id2);
		       continue 'other_ships }
	    };
            if ship_coll.check_collision(&ship_body.position,
					 (ship_coll2,&ship_body2.position)) {
		events.push(InputEvent::Collision(*ship_id,*ship_id2));
	    }
            // NOTE: previous logic denotes the requirement for colliders
            // even though below function does not require it

            if (zoom > 1.0) &
                Collider::check_collision_zoomed(&ship_body.position,
					         &ship_body2.position,
                                                 zoom) {
		    events.push(InputEvent::VisualCollision(*ship_id,*ship_id2));
		}
	}
    }
}






pub struct Headless {
    events  : Vec<InputEvent>,
    receiver: Receiver<InputEvent>,
}

impl Interface for Headless {
    fn new(_: Config) -> io::Result<Headless> {
        let (sender, receiver) = channel();

        spawn(move || -> () {
            let stdin = stdin();

            loop {
                let mut line = String::new();
                match stdin.read_line(&mut line) {
                    Ok(_) => match InputEvent::from_json(line.as_ref()) {
                        Ok(event) =>
                            match sender.send(event) {
                                Ok(()) =>
                                    (),
                                Err(error) =>
                                    panic!("Error sending input: {:?}", error),
                            },
                        Err(error) =>
                            panic!("Error decoding input: {:?}", error),
                    },
                    Err(error) =>
                        panic!("Error reading from stdin: {}", error),
                }
            }
        });

        Ok(Headless {
            events  : Vec::new(),
            receiver: receiver,
        })
    }

    fn update(&mut self, frame: &mut Frame) -> io::Result<Drain<InputEvent>> {
        loop {
            match self.receiver.try_recv() {
                Ok(event) =>
                    self.events.push(event),
                Err(error) => match error {
                    TryRecvError::Empty        => break,
                    TryRecvError::Disconnected => panic!("Channel disconnected"),
                }
            }
        }

        print!("{}\n", frame.to_json());

        Ok(self.events.drain(..))
    }

    fn get_config (&self) -> Option<Config> { None }
}
