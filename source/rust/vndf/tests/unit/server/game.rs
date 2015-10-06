use vndf::server::game::state::{GameState};
use vndf::shared::game::{
	Body,
	EntityId,
	ManeuverData,
};
use vndf::shared::util::angle_of;


#[test]
fn it_should_execute_multiple_maneuvers_after_each_other() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let maneuver_a = ManeuverData {
		start_s   : 0.5,
		duration_s: 0.05,
		angle     : 1.0,
		thrust    : 1.0,
	};
	let maneuver_b = ManeuverData {
		start_s   : 1.0,
		duration_s: 0.05,
		angle     : -1.0,
		thrust    : 1.0,
	};

	game_state.on_schedule_maneuver(ship_id, maneuver_a);
	game_state.on_schedule_maneuver(ship_id, maneuver_b);

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver_a.start_s + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver_a.angle, before, after));

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver_b.start_s + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver_b.angle, before, after));
}

#[test]
fn maneuvers_should_apply_thrust_over_time() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let maneuver = ManeuverData {
		start_s   : 0.5,
		duration_s: 0.2,
		angle     : 1.0,
		thrust    : 1.0,
	};

	game_state.on_schedule_maneuver(ship_id, maneuver);

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver.start_s + maneuver.duration_s / 2.0);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver.angle, before, after));

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver.start_s + maneuver.duration_s);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver.angle, before, after));
}


#[test]
fn maneuver_thrust_should_be_configurable() {
	let mut game_state = GameState::new();

	let ship_id_a = game_state.on_enter();
	let ship_id_b = game_state.on_enter();

	let start_s    = 0.5;
	let duration_s = 1.0;
	let angle      = 0.0;

	let maneuver_a = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : 1.0,
	};
	let maneuver_b = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : 0.5,
	};

	game_state.on_schedule_maneuver(ship_id_a, maneuver_a);
	game_state.on_schedule_maneuver(ship_id_b, maneuver_b);
	game_state.on_update(start_s + duration_s);

	let body_a = get_body(ship_id_a, &mut game_state);
	let body_b = get_body(ship_id_b, &mut game_state);

	assert!(body_a.velocity.x > body_b.velocity.x);
}

#[test]
fn maneuver_thrust_should_stay_within_limits() {
	let mut game_state = GameState::new();

	let ship_id_a = game_state.on_enter();
	let ship_id_b = game_state.on_enter();
	let ship_id_c = game_state.on_enter();
	let ship_id_d = game_state.on_enter();

	let start_s    = 0.5;
	let duration_s = 1.0;
	let angle      = 0.0;

	let maneuver_a = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : 1.0,
	};
	let maneuver_b = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : 2.0,
	};
	let maneuver_c = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : 0.0,
	};
	let maneuver_d = ManeuverData {
		start_s   : start_s,
		duration_s: duration_s,
		angle     : angle,
		thrust    : -1.0,
	};

	game_state.on_schedule_maneuver(ship_id_a, maneuver_a);
	game_state.on_schedule_maneuver(ship_id_b, maneuver_b);
	game_state.on_schedule_maneuver(ship_id_c, maneuver_c);
	game_state.on_schedule_maneuver(ship_id_d, maneuver_d);
	game_state.on_update(start_s + duration_s);

	let body_a = get_body(ship_id_a, &mut game_state);
	let body_b = get_body(ship_id_b, &mut game_state);
	let body_c = get_body(ship_id_c, &mut game_state);
	let body_d = get_body(ship_id_d, &mut game_state);

	assert!(body_a.velocity.x == body_b.velocity.x);
	assert!(body_c.velocity.x == body_d.velocity.x);
}

#[test]
fn players_should_only_be_able_to_cancel_their_own_maneuvers() {
	let mut game_state = GameState::new();

	let ship_id_a = game_state.on_enter();
	let ship_id_b = game_state.on_enter();

	let maneuver = ManeuverData {
		start_s   : 0.5,
		duration_s: 1.0,
		angle     : 0.0,
		thrust    : 1.0,
	};

	game_state.on_schedule_maneuver(ship_id_a, maneuver);
	game_state.on_schedule_maneuver(ship_id_b, maneuver);

	assert_eq!(game_state.entities.maneuvers.len(), 2);

	let maneuver_id_a = get_maneuver_id(ship_id_a, &mut game_state);
	game_state.on_cancel_maneuver(ship_id_b, maneuver_id_a);
	game_state.on_update(0.0);

	assert_eq!(game_state.entities.maneuvers.len(), 2);
}


fn get_body(body_id: EntityId, game_state: &mut GameState) -> Body {
	for entity in game_state.export_entities() {
		if entity.id == body_id {
			return entity.body.unwrap();
		}
	}

	unreachable!();
}

fn get_maneuver_id(ship_id: EntityId, game_state: &mut GameState) -> EntityId {
	for (id, maneuver) in &game_state.entities.maneuvers {
		if ship_id == maneuver.ship_id {
			return *id;
		}
	}

	panic!("Maneuver not found");
}

fn angle_has_decreased(direction: f64, before: Body, after: Body, ) -> bool {
	let old_difference = (direction - angle_of(before.velocity)).abs();
	let new_difference = (direction - angle_of(after.velocity )).abs();

	new_difference < old_difference
}
