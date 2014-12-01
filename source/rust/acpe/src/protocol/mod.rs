use std::io::IoResult;


pub use self::action::Action;
pub use self::decode::decode;
pub use self::encode::{
	Encoder,
	MessageEncoder,
};
pub use self::perception::Perception;


mod action;
mod decode;
mod encode;
mod perception;


pub type Seq = u64;


pub trait MessagePart {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;

	// TODO: This interface doesn't allow for an allocation-free implementation,
	//       when the type contains a String, Vec, or similar.
	fn read(line: &str) -> Result<Self, String>;
}
