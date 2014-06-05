use collections::HashMap;

use common::ecs::components::{
	MissileVisual,
	ShipVisual,
	Visual
};
use common::ecs::infra::{
	Components,
	EntityId,
};
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::ShipControl;
use network::ClientId;


entity!(Missile<Body, Visual>, |args: (Vec2, Radians)| {
	let (position, attitude) = args;

	let body = Body {
		position: position,
		velocity: Vec2::zero(),
		attitude: attitude
	};

	(body, MissileVisual)
})

entity!(Ship<Body, ShipControl, Visual>, |client_id: ClientId| {
	let body = Body {
		position: Vec2::zero(),
		velocity: Vec2::zero(),
		attitude: Radians(0.0)
	};

	let ship_control = ShipControl {
		client_id    : client_id,
		missile_index: 0
	};

	(body, ship_control, ShipVisual)
})


pub struct Entities {
	next_id: EntityId,

	pub bodies       : Components<Body>,
	pub ship_controls: Components<ShipControl>,
	pub visuals      : Components<Visual>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			next_id: 0,

			bodies       : HashMap::new(),
			ship_controls: HashMap::new(),
			visuals      : HashMap::new(),
		}
	}

	pub fn entity_id_from_client_id(&self, client_id: ClientId) -> Option<EntityId> {
		for (&id, ship_control) in self.ship_controls.iter() {
			if ship_control.client_id == client_id {
				return Some(id);
			}
		}

		None
	}

	pub fn create_ship(&mut self, client_id: ClientId) {
		let id = self.next_id();

		Ship::create(
			id,
			client_id,
			&mut self.bodies,
			&mut self.ship_controls,
			&mut self.visuals);
	}

	pub fn destroy_ship(&mut self, client_id: ClientId) {
		let id = match self.entity_id_from_client_id(client_id) {
			Some(id) => id,
			None     => return
		};

		Ship::destroy(
			id,
			&mut self.bodies,
			&mut self.ship_controls,
			&mut self.visuals);
	}

	pub fn create_missile(&mut self, position: Vec2, attitude: Radians) {
		let id = self.next_id();

		Missile::create(
			id,
			(position, attitude),
			&mut self.bodies,
			&mut self.visuals);
	}

	fn next_id(&mut self) -> EntityId {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}
