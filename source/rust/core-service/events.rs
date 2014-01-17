use std::f64;
use std::libc;
use std::ptr;

extern {
	fn close(fd: libc::c_int) -> libc::c_int;
}


pub static ON_CONNECT: libc::c_int    = 0;
pub static ON_DISCONNECT: libc::c_int = 1;
pub static ON_UPDATE: libc::c_int     = 2;

pub struct Event {
	theType: libc::c_int,

	onConnect   : ConnectEvent,
	onDisconnect: DisconnectEvent,
	onUpdate    : UpdateEvent
}

pub struct ConnectEvent {
	clientFD: libc::c_int
}

pub struct DisconnectEvent {
	clientId: uint
}

pub struct UpdateEvent {
	dummy: libc::c_int
}

pub struct Events {
	first : u64,
	last  : u64,
	cap   : libc::size_t,
	buffer: *mut Event
}


pub fn handle_events(events: &mut Events, clientMap: &mut ::clients::ClientMap, frameTimeInMs: libc::c_int) {
	unsafe {
		while (events.last - events.first > 0) {
			let event = *(ptr::mut_offset(events.buffer, (events.first % events.cap) as int));
			events.first += 1;

			match event.theType {
				ON_CONNECT    => on_connect(event.onConnect.clientFD, clientMap),
				ON_DISCONNECT => on_disconnect(event.onDisconnect.clientId as uint, clientMap, events),
				ON_UPDATE     => on_update(clientMap, events, frameTimeInMs as f64 / 1000.0),

				_ => assert!(false)
			}
		}
	}
}

fn on_connect(clientFD: libc::c_int, clientMap: &mut ::clients::ClientMap) {
	if (::clients::can_add(clientMap)) {
		let distance = 100.0;

		let alpha = 90.0 / 180.0 * f64::consts::PI;

		let pos = ::common::vec::Vec2 {
			x: distance * f64::cos(alpha),
			y: distance * f64::sin(alpha) };

		let vel = ::common::vec::Vec2 {
			x: 30.0,
			y: 0.0 };

		::clients::add(clientMap, clientFD, pos, vel);
	}
	else
	{
		unsafe {
			close(clientFD);
		}
	}
}

fn on_disconnect(clientId: uint, clientMap: &mut ::clients::ClientMap, events: &mut Events) {
	::clients::remove(clientMap, clientId);

	clientMap.clients.each(|client| {
		let status = ::protocol::send_remove(
			client.socketFD,
			clientId as u64);

		if (status < 0) {
			let disconnectEvent = Event {
				theType: ON_DISCONNECT,
				onDisconnect: DisconnectEvent {
					clientId: client.id },
				onConnect: ConnectEvent { clientFD: 0 },
				onUpdate: UpdateEvent { dummy: 0 } };

			unsafe {
				let ptr = ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
				*ptr = disconnectEvent;
				events.last += 1;
			}
		}
	})
}

fn on_update(clientMap: &mut ::clients::ClientMap, events: &mut Events, dTimeInS: f64) {
	clientMap.clients.each(|client| {
		let gMag = 3000.0 / client.ship.pos.magnitude();
		let g = client.ship.pos.normalize() * -gMag;

		client.ship.pos = client.ship.pos + client.ship.vel * dTimeInS;
		client.ship.vel = client.ship.vel + g * dTimeInS;
	});

	clientMap.clients.each(|clientA| {
		clientMap.clients.each(|clientB| {
			let status = ::protocol::send_update(
				clientA.socketFD,
				clientB.id as u64,
				clientB.ship.pos.x,
				clientB.ship.pos.y);

			if (status < 0) {
				let disconnectEvent = Event {
					theType: ON_DISCONNECT,
					onDisconnect: DisconnectEvent {
						clientId: clientA.id },
					onConnect: ConnectEvent { clientFD: 0 },
					onUpdate: UpdateEvent { dummy: 0 } };

				unsafe {
					let ptr = ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
					*ptr = disconnectEvent;
				}
				events.last += 1;
			}
		})
	});
}
