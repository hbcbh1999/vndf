use std::old_io::IoResult;

use client::platform::Message;
use common::game::Broadcast;
use render::{
	draw_border,
	Pos,
	ScreenBuffer,
};
use render::C;
use render::Color::{
	Green,
	Red,
};

use super::base::{
	Render,
	Status,
};
use super::state::{
	InfoSection,
	MainSection,
	TabSwitcher,
};


pub fn button(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, text: &str) -> IoResult<()> {
	let (foreground_color, background_color) = status.colors();

	buffer
		.writer(x, y)
		.foreground_color(foreground_color)
		.background_color(background_color)
		.write_str(text)
}

pub fn list(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, width: Pos, height: Pos, first: usize, items: &[String]) -> IoResult<()> {
	let limit = x + width;

	let (foreground_color, background_color) = status.colors();

	let items: Vec<String> = if items.len() == 0 {
		vec!["none".to_string()]
	}
	else {
		items
			.iter()
			.map(|s| s.clone())
			.collect()
	};

	let mut iter = items
		.iter()
		.skip(first);

	for i in range(0, height) {
		let item_length = match iter.next() {
			Some(item) => {
				try!(
					buffer
						.writer(x, y + i as Pos)
						.limit(limit)
						.foreground_color(foreground_color)
						.background_color(background_color)
						.write_str(item.as_slice())
				);

				item.chars().count()
			},
			None =>
				0,
		};

		for x in range(x + item_length as Pos, limit - 1) {
			try!(
				buffer
					.writer(x, y + i as Pos)
					.limit(limit)
					.foreground_color(foreground_color)
					.background_color(background_color)
					.write_char(' ')
			);
		}
	}

	if first > 0 {
		try!(write!(
			&mut buffer.writer(limit - 1, y).limit(limit),
			"↑",
		));
	}

	if items.len() - first > height as usize {
		try!(write!(
			&mut buffer.writer(limit - 1, y + height - 1).limit(limit),
			"↓",
		));
	}

	Ok(())
}

pub fn tab_header(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, label: &str) -> IoResult<()> {
	let (foreground_color, background_color) = status.colors();

	buffer
		.writer(x, y)
		.foreground_color(foreground_color)
		.background_color(background_color)
		.write_str(label)
}

pub fn text_field(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, width: Pos, text: &str) -> IoResult<()> {
	let limit = x + width;

	let (foreground_color, background_color) = status.colors();

	try!(
		buffer
			.writer(x, y)
			.limit(limit)
			.foreground_color(foreground_color)
			.background_color(background_color)
			.write_str(text)
	);
	for x in range(x + text.chars().count() as Pos, limit) {
		try!(
			buffer
				.writer(x, y)
				.limit(limit)
				.foreground_color(foreground_color)
				.background_color(background_color)
				.write_str(" ")
		);
	}

	buffer.cursor = if status == Status::Active {
		Some((1 + x + text.chars().count() as Pos, 1 + y))
	}
	else {
		None
	};

	Ok(())
}


impl Render for InfoSection {
	type Args = Message;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		status: &Message,
	)
		-> IoResult<()>
	{
		try!(draw_border(
			buffer,
			x, y,
			self.width,
			self.height
		));

		{
			let status_writer = buffer.writer(x + 1, y + 1);

			let (mut status_writer, status) = match *status {
				Message::Notice(ref s) =>
					(status_writer.foreground_color(Green), s.as_slice()),
				Message::Error(ref s) =>
					(status_writer.foreground_color(Red  ), s.as_slice()),
				Message::None =>
					(status_writer, ""),
			};

			try!(write!(
				&mut status_writer,
				"{}",
				status
			));
		}

		Ok(())
	}
}


pub struct MainSectionArgs<'a> {
	pub broadcasts: &'a [Broadcast],
}

impl<'a> Render for MainSection {
	type Args = MainSectionArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &MainSectionArgs,
	)
		-> IoResult<()>
	{
		try!(draw_border(
			buffer,
			x, y,
			self.width,
			self.height,
		));

		let mut broadcasts: Vec<String> = args.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		try!(self.tab_switcher.render(buffer, x + 1, y + 1, &()));

		Ok(())
	}
}


impl<'a> Render for TabSwitcher {
	type Args = ();

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		_     : &(),
	)
		-> IoResult<()>
	{
		let mut c = C::new();
		c.c = '─';
		for x in range(x, buffer.width()) {
			try!(buffer.set(x, y + 1, c));
		}

		Ok(())
	}
}
