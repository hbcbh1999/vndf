use collections::HashMap;

use common::ecs::components::Ship;
use common::testing::{
	Client,
	MockGameService
};
use common::physics::{
	Body,
	Radians,
	Vec2
};
use common::physics::util;
use common::protocol::{
	Entities,
	Perception
};


#[test]
fn it_should_interpolate_between_perceptions() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = Vec2::zero();
	let pos_2 = Vec2(10.0, 0.0);

	let mut perception_1 = Perception {
		self_id : 0,
		updated: Entities {
			bodies  : HashMap::new(),
			ships   : HashMap::new(),
			missiles: HashMap::new()
		}
	};
	perception_1.updated.bodies.insert(0, Body {
		position: pos_1,
		velocity: Vec2(10.0, 0.0),
		attitude: Radians(0.0)
	});
	perception_1.updated.ships.insert(0, Ship::new());
	let mut perception_2 = perception_1.clone();
	perception_2.updated.bodies.get_mut(&0).position = pos_2;

	game_service.send_perception(&perception_1);
	game_service.send_perception(&perception_2);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships.get(0).position == pos_1 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	assert!(util::is_on_line(
		(pos_1, pos_2),
		frame_1.ships.get(0).position,
		16));
	assert!(util::is_on_line(
		(pos_1, pos_2),
		frame_2.ships.get(0).position,
		16));
	assert!(frame_2.ships.get(0).position != pos_2);
}


#[test]
fn the_camera_should_follow_the_ship() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = Vec2::zero();
	let pos_2 = Vec2(10.0, 0.0);

	let mut perception_1 = Perception {
		self_id : 0,
		updated: Entities {
			bodies  : HashMap::new(),
			ships   : HashMap::new(),
			missiles: HashMap::new()
		}
	};
	perception_1.updated.bodies.insert(0, Body {
		position: pos_1,
		velocity: Vec2(10.0, 0.0),
		attitude: Radians(0.0)
	});
	perception_1.updated.ships.insert(0, Ship::new());
	let mut perception_2 = perception_1.clone();
	perception_2.updated.bodies.get_mut(&0).position = pos_2;

	game_service.send_perception(&perception_1);
	let mut frame_1 = client.frame();

	game_service.send_perception(&perception_2);
	let mut frame_2 = client.frame();

	while frame_2.camera == pos_1 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	assert_eq!(
		pos_1,
		frame_1.camera);
	assert_eq!(
		pos_2,
		frame_2.camera);
}
