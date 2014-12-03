#![feature(slicing_syntax)]


extern crate acpe;

extern crate common;
extern crate game_service;


use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::timer::sleep;
use std::time::Duration;

use acpe::MAX_PACKET_SIZE;
use acpe::protocol::{
	Encoder,
	PerceptionHeader,
	Seq,
};

use common::protocol::{
	Broadcast,
	Percept,
	Step,
};
use game_service::Socket;


struct Client {
	id         : String,
	last_action: Seq,
	broadcast  : Option<String>,
}


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashMap::new();
	let mut socket  = Socket::new(port);
	let mut encoder = Encoder::new();

	print!("Listening on port {}\n", port);

	loop {
		let received = socket.recv_from();
		for result in received.into_iter() {
			match result {
				Ok((action, address)) => {
					for step in action.steps.into_iter() {
						match step {
							Step::Login => {
								clients.insert(address, Client {
									// TODO: Set id
									id         : "".to_string(),
									last_action: action.header.id,
									broadcast  : None,
								});
							},
							Step::Broadcast(broadcast) => {
								clients[address].broadcast = Some(broadcast);
							},
						}
					}

					clients[address].last_action = action.header.id;
				},
				Err((error, address)) => {
					print!("Error receiving message from {}: {}", address, error);
					clients.remove(&address);
				},
			}
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, client)|
					client.broadcast.clone()
			)
			.collect();

		for (&address, client) in clients.iter() {
			let header = PerceptionHeader {
				confirm_action: client.last_action,
				self_id       : Some(client.id.clone()),
			};
			let mut broadcasts: Vec<&str> = broadcasts
				.iter()
				.map(|broadcast| broadcast.as_slice())
				.collect();

			let mut needs_to_send_perception = true;
			while needs_to_send_perception {
				send_perception(
					&mut encoder,
					&header,
					&mut broadcasts,
					&mut socket,
					address,
				);

				needs_to_send_perception = broadcasts.len() > 0;
			}
		}

		sleep(Duration::milliseconds(20));
	}
}


fn send_perception(
	encoder    : &mut Encoder,
	header     : &PerceptionHeader,
	broadcasts : &mut Vec<&str>,
	socket     : &mut Socket,
	address    : SocketAddr,
) {
	let mut perception = encoder.message(header);
	loop {
		let message = match broadcasts.pop() {
			Some(message) => message,
			None          => break,
		};

		// TODO: Set sender
		let broadcast = Broadcast {
			sender : "".to_string(),
			message: message.to_string(),
		};

		if !perception.add(&Percept::Broadcast(broadcast)) {
			broadcasts.push(message);
			break;
		}
	}

	let mut encode_buffer = [0, ..MAX_PACKET_SIZE];

	let message = perception
		.encode(&mut encode_buffer)
		.unwrap_or_else(|error|
			panic!("Error encoding perception: {}", error)
		);
	socket.send_to(message, address);
}
