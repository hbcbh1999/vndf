use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;

use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use game_service::network::Network;


pub struct GameService {
	port    : Port,
	network : Network,
	incoming: Vec<(SocketAddr, ClientEvent)>,
}

impl GameService {
	pub fn start() -> GameService {
		let port    = random_port(40000, 50000);
		let network = Network::new(port);

		GameService {
			port    : port,
			network : network,
			incoming: Vec::new(),
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_event(&mut self) -> Option<ClientEvent> {
		let start_s = precise_time_s();

		while self.incoming.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.incoming.extend(self.network.receive());
		}

		if self.incoming.len() > 0 {
			let (address, event) = self.incoming.remove(0);

			// This makes sure that confirmations are sent back to the client.
			// TODO: Remove
			self.network.send(
				Some(address).into_iter(),
				Some(ServerEvent::SelfId("".to_string())).into_iter(),
			);

			Some(event)
		}
		else {
			None
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until<F>(&mut self, condition: F) -> Option<ClientEvent>
		where
			F: Fn(&mut Option<ClientEvent>) -> bool,
	{
		let start_s = precise_time_s();

		let mut event = self.expect_event();

		while !condition(&mut event) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			event = self.expect_event();
		}

		event
	}
}
