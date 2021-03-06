use nalgebra::{
    Norm,
    Rot2,
    Rotate,
    Vec1,
    Vec2,
};

use server::game::state::GameState;
use shared::game::logic;


pub fn apply_maneuvers(game_state: &mut GameState, now_s: f64) {
    for (&id, maneuver) in &mut game_state.entities.maneuvers {
        if now_s >= maneuver.data.start_s {
            let rotation = Rot2::new(Vec1::new(maneuver.data.angle));
            let force    = rotation.rotate(&Vec2::new(1.0, 0.0));
            let force    = force * maneuver.data.thrust;

            match game_state.entities.bodies.get_mut(&maneuver.ship_id) {
                Some(body) =>
                    body.force = body.force + force,

                // The ship might not exist due to timing issues (it could
                // have been destroyed while the message was in flight). If
                // this happens too often, it might also be the symptom of a
                // bug.
                None => debug!("Ship not found: {}", maneuver.ship_id),
            }
        }

        if now_s >= maneuver.data.start_s + maneuver.data.duration_s {
            game_state.to_destroy.push(id);
        }
    }
}

pub fn apply_gravity(game_state: &mut GameState) {
    for (_, planet) in &game_state.entities.planets {
        for (_, body) in &mut game_state.entities.bodies {
            logic::apply_gravity(planet, body);
        }
    }
}

pub fn move_bodies(game_state: &mut GameState, delta_t_s: f64) {
     for (_, body) in &mut game_state.entities.bodies {
        logic::integrate(body, delta_t_s);
    }
}

pub fn check_collisions(game_state: &mut GameState) {
    for (&body_id, body) in &game_state.entities.bodies {
        for (_, planet) in &game_state.entities.planets {
            let squared_radius = planet.radius * planet.radius;

            if (body.position - planet.position).sqnorm() < squared_radius {
                game_state.to_destroy.push(body_id);
            }
        }
    }
}
