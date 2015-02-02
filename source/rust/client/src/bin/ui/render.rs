use std::cmp::max;
use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};

use super::data::{
	BroadcastForm,
	Button,
	CommTab,
	List,
	Status,
	TextField,
};


pub trait Render {
	type Args;

	fn render(&self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &Self::Args)
		-> IoResult<()>;
}


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

pub struct BroadcastFormArgs {
	pub status : Status,
	pub sending: bool,
}

impl Render for BroadcastForm {
	type Args = BroadcastFormArgs;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &BroadcastFormArgs,
	)
		-> IoResult<()>
	{
		let button_text = if args.sending {
			STOP_BROADCAST
		}
		else {
			START_BROADCAST
		};

		let width = buffer.width() - x;
		let button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;
		let broadcast_width = width - 2 - button_width - 2;

		let text_field_status =
			if args.status == Status::Active && !args.sending {
				Status::Active
			}
			else if args.status == Status::Active {
				Status::Selected
			}
			else {
				args.status
			};

		try!(self.text_field.render(
			buffer,
			x, y,
			&TextFieldArgs {
				width : broadcast_width,
				status: text_field_status,
			},
		));

		let button_status =
			if args.status == Status::Selected || args.status == Status::Active {
				Status::Active
			}
			else {
				Status::Passive
			};

		try!(self.button.render(
			buffer,
			x + broadcast_width + 2, y,
			&ButtonArgs {
				text  : button_text,
				status: button_status,
			},
		));

		Ok(())
	}
}


pub struct ButtonArgs<'a> {
	pub text  : &'a str,
	pub status: Status,
}

impl<'a> Render for Button {
	type Args = ButtonArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &ButtonArgs,
	)
		-> IoResult<()>
	{
		let (foreground_color, background_color) = args.status.colors();

		buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color)
			.write_str(args.text)
	}
}


pub struct CommTabArgs<'a> {
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
	pub is_sending: bool,
}

impl<'a> Render for CommTab {
	type Args = CommTabArgs<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &CommTabArgs,
	)
		-> IoResult<()>
	{
		try!(write!(
			&mut buffer.writer(x, y),
			"YOUR ID",
		));

		try!(write!(
			&mut buffer.writer(x + 4, y + 1),
			"{}",
			args.self_id,
		));

		try!(write!(
			&mut buffer.writer(x, y + 3),
			"SENDING",
		));

		let form_status = if self.form_is_selected() && self.element_active {
			Status::Active
		}
		else if self.form_is_selected() {
			Status::Selected
		}
		else {
			Status::Passive
		};

		try!(self.broadcast_form.render(
			buffer,
			x + 4, y + 4,
			&BroadcastFormArgs {
				status : form_status,
				sending: args.is_sending,
			},
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let list_status = if self.list_is_selected() && self.element_active {
			Status::Active
		}
		else if self.list_is_selected() {
			Status::Selected
		}
		else {
			Status::Passive
		};

		let width = buffer.width();
		try!(self.broadcast_list.render(
			buffer,
			x + 4, y + 7,
			&ListArgs {
				width : width - 4 - 4,
				height: 5,
				items : args.broadcasts,
				status: list_status,
			},
		));

		Ok(())
	}
}


pub struct ListArgs<'a> {
	pub width : Pos,
	pub height: Pos,
	pub items : &'a [String],
	pub status: Status,
}

impl<'a> Render for List {
	type Args = ListArgs<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &ListArgs,
	)
		-> IoResult<()>
	{
		let limit = x + args.width;

		let (foreground_color, background_color) = args.status.colors();

		let items: Vec<String> = if args.items.len() == 0 {
			vec!["none".to_string()]
		}
		else {
			args.items
				.iter()
				.map(|s| s.clone())
				.collect()
		};

		let mut iter = items
			.iter()
			.skip(self.first);

		for i in range(0, args.height) {
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

		if self.first > 0 {
			try!(write!(
				&mut buffer.writer(limit - 1, y).limit(limit),
				"↑",
			));
		}

		if items.len() - self.first > args.height as usize {
			try!(write!(
				&mut buffer.writer(limit - 1, y + args.height - 1).limit(limit),
				"↓",
			));
		}

		Ok(())
	}
}


pub struct TextFieldArgs {
	pub width : Pos,
	pub status: Status,
}

impl Render for TextField {
	type Args = TextFieldArgs;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &TextFieldArgs,
	)
		-> IoResult<()>
	{
		let text  = self.text.as_slice();
		let limit = x + args.width;

		let (foreground_color, background_color) = args.status.colors();

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

		buffer.cursor = if args.status == Status::Active {
			Some((1 + x + text.chars().count() as Pos, 1 + y))
		}
		else {
			None
		};

		Ok(())
	}
}
