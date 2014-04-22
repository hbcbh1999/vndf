use libc;
use libc::c_int;
use std::mem;
use std::ptr;

use net::Connection;
use net::ffi;
use net::epoll::EPoll;
use util::last_error;


pub struct Acceptor {
	pub fd: c_int,

	epoll: EPoll
}

impl Acceptor {
	pub fn create(port: &str, epoll: EPoll) -> Acceptor {
		let fd = init_socket(port);

		Acceptor {
			fd   : fd,
			epoll: epoll
		}
	}

	pub fn accept(&self) -> Result<Connection, ~str> {
		let fd = unsafe {
			ffi::accept(
				self.fd,
				ptr::mut_null(),
				ptr::mut_null())
		};

		if fd >= 0 {
			Ok(Connection::from_fd(fd))
		}
		else {
			Err(last_error())
		}
	}
}


fn init_socket(port: &str) -> c_int {
	let hints = ffi::addrinfo {
		ai_flags    : ffi::AI_PASSIVE,
		ai_family   : ffi::AF_UNSPEC,
		ai_socktype : ffi::SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ptr::null(),
		ai_canonname: ptr::null(),
		ai_next     : ptr::null()
	};

	let servinfo: *ffi::addrinfo = ptr::null();

	unsafe {
		let status = port.to_c_str().with_ref(|c_message| {
			ffi::getaddrinfo(
				ptr::null(),
				c_message,
				&hints,
				&servinfo)
		});

		if status != 0 {
			fail!("Error getting address info: {}", last_error());
		}

		let socket_fd = ffi::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if socket_fd == -1 {
			fail!("Error creating socket: {}", last_error());
		}

		let yes = 1;
		let status = ffi::setsockopt(
			socket_fd,
			ffi::SOL_SOCKET,
			ffi::SO_REUSEADDR,
			&yes as *int as *libc::c_void,
			mem::size_of::<c_int>() as u32);

		if status == -1 {
			fail!("Error setting socket option: {}", last_error());
		}

		let status = ffi::bind(
			socket_fd,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			fail!("Error binding socket: {}", last_error());
		}

		let status = ffi::listen(
			socket_fd,
			1024);
		if status != 0 {
			fail!("Error listening on socket: {}", last_error());
		}

		ffi::freeaddrinfo(servinfo);

		socket_fd
	}
}
