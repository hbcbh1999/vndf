#![feature(slicing_syntax)]


extern crate "rustc-serialize" as rustc_serialize;


pub use constants::{
	DESTROY,
	MAX_PACKET_SIZE,
	UPDATE,
};


pub mod network;
pub mod protocol;


mod constants {
	pub const MAX_PACKET_SIZE: uint = 512;

	pub const DESTROY: &'static str = "DESTROY";
	pub const UPDATE : &'static str = "UPDATE";
}
