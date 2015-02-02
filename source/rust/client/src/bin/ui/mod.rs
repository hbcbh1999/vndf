pub mod data;
pub mod input;
pub mod render;


use std::vec::Drain;

use client::platform::{
	Frame,
	InputEvent,
};

use self::data::CommTab;
use self::input::{
	Direction,
	ProcessInput,
};


pub struct Ui {
	pub comm_tab: CommTab,

	mode  : TextInputMode,
	events: Vec<InputEvent>,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			comm_tab: CommTab::new(),
			mode    : TextInputMode::Regular,
			events  : Vec::new(),
		}
	}

	pub fn process_input(&mut self, frame: &Frame, chars: &[char])
		-> Drain<InputEvent>
	{
		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else {
						self.comm_tab.process_char(c);
					}
				},
				TextInputMode::Escape => {
					if c == '[' {
						self.mode = TextInputMode::Cursor;
					}
					else {
						// Unexpected character. Fall back to regular mode.
						self.mode = TextInputMode::Regular;
					}
				},
				TextInputMode::Cursor => {
					let direction = match c {
						'A' => Some(Direction::Up),
						'B' => Some(Direction::Down),
						'C' => Some(Direction::Right),
						'D' => Some(Direction::Left),
						_   => None, // Unexpected character
					};

					if let Some(direction) = direction {
						self.comm_tab.process_cursor(direction);
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}

		let is_sending = frame.broadcasts
			.iter()
			.any(|broadcast|
				broadcast.sender == frame.self_id
			);

		if self.comm_tab.broadcast_form.button.was_activated {
			self.comm_tab.broadcast_form.button.was_activated = false;

			if is_sending {
				self.events.push(InputEvent::StopBroadcast);
			}
			else {
				let message =
					self.comm_tab.broadcast_form.text_field.text.clone();
				self.events.push(InputEvent::StartBroadcast(message));
			}
		}

		self.events.drain()
	}
}


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}
