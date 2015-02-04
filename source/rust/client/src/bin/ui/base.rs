use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};
use render::Color::{
	self,
	Black,
	Blue,
	White,
	Yellow,
};


pub trait ProcessInput {
	fn process_event(&mut self, event: InputEvent);
}

pub enum InputEvent {
	Char(char),

	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,

	Backspace,
	Enter,
}


pub trait Update {
	type Args;

	fn update(&mut self, args: &Self::Args);
}


pub trait Render {
	type Args;

	fn render(&self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &Self::Args)
		-> IoResult<()>;
}


#[derive(Copy, Eq, PartialEq)]
pub enum Status {
	Passive,
	Selected,
	Active,
}

impl Status {
	pub fn colors(&self) -> (Color, Color) {
		match *self {
			Status::Passive  => (White, Black ),
			Status::Selected => (White, Blue  ),
			Status::Active   => (Black, Yellow),
		}
	}
}