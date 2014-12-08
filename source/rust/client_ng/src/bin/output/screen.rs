use libc;
use std::io::{
	stdout,
	IoError,
	IoErrorKind,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;
use std::mem::swap;
use std::str::from_utf8;

use termios::Termios;


pub struct Screen {
	stdout  : LineBufferedWriter<StdWriter>,
	buffer_a: ScreenBuffer,
	buffer_b: ScreenBuffer,
	cursor  : (u16, u16),
}

impl Screen {
	pub fn new(width: u16, height: u16) -> IoResult<Screen> {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		let mut stdout = stdout();
		match write!(&mut stdout, "\x1b[2J") { // clear screen
			Ok(())     => (),
			Err(error) => return Err(error),
		}

		let buffer_a = ScreenBuffer::new(width, height);
		let buffer_b = buffer_a.clone();

		Ok(Screen {
			stdout  : stdout,
			buffer_a: buffer_a,
			buffer_b: buffer_b,
			cursor  : (0, 0),
		})
	}

	pub fn width(&self) -> u16 {
		self.buffer_a.width()
	}

	/// Origin is in upper-left corner.
	pub fn buffer(&mut self, x: u16, y: u16, limit: u16,) -> BufferWriter {
		BufferWriter {
			buffer: &mut self.buffer_a,
			x     : x,
			y     : y,
			limit : x + limit,
		}
	}

	pub fn set_cursor(&mut self, x: u16, y: u16) {
		self.cursor = (x, y);
	}

	pub fn submit(&mut self) -> IoResult<()> {
		{
			let mut iter = self.buffer_a.iter().zip(self.buffer_b.iter());
			for ((x, y, c_a), (_, _, c_b)) in iter {
				if c_a != c_b {
					try!(write!(
						&mut self.stdout,
						"\x1b[{};{}H", // move cursor
						y + 1, x + 1
					));
					try!(self.stdout.write_char(c_a));
				}
			}
		}

		swap(&mut self.buffer_a, &mut self.buffer_b);
		self.buffer_a.clear();

		let (x, y) = self.cursor;
		try!(write!(
			&mut self.stdout,
			"\x1b[{};{}H", // set cursor
			y + 1, x + 1
		));

		try!(self.stdout.flush());
		Ok(())
	}
}


#[deriving(Clone)]
struct ScreenBuffer {
	text: Vec<Vec<char>>,
}

impl ScreenBuffer {
	pub fn new(width: u16, height: u16) -> ScreenBuffer {
		let width  = width  as uint;
		let height = height as uint;

		ScreenBuffer {
			text: Vec::from_fn(height, |_| Vec::from_elem(width, ' '))
		}
	}

	pub fn width(&self) -> u16 {
		self.text[0].len() as u16
	}

	pub fn height(&self) -> u16 {
		self.text.len() as u16
	}

	pub fn iter(&self) -> BufferIterator {
		BufferIterator {
			buffer: &self.text,
			x     : 0,
			y     : 0,
		}
	}

	pub fn clear(&mut self) {
		for line in self.text.iter_mut() {
			for c in line.iter_mut() {
				*c = ' ';
			}
		}
	}
}


struct BufferIterator<'r> {
	buffer: &'r Vec<Vec<char>>,
	x     : uint,
	y     : uint,
}

impl<'r> Iterator<(u16, u16, char)> for BufferIterator<'r> {
	fn next(&mut self) -> Option<(u16, u16, char)> {
		if self.x >= self.buffer[0].len() {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.buffer.len() {
			return None;
		}

		let result =
			Some((self.x as u16, self.y as u16, self.buffer[self.y][self.x]));

		self.x += 1;

		result
	}
}


struct BufferWriter<'r> {
	buffer: &'r mut ScreenBuffer,
	x     : u16,
	y     : u16,
	limit : u16,
}

impl<'r> Writer for BufferWriter<'r> {
	fn write(&mut self, buf: &[u8]) -> IoResult<()> {
		if self.y >= self.buffer.height() {
			return Err(IoError {
				kind  : IoErrorKind::OtherIoError,
				desc  : "y coordinate is out of bounds",
				detail: None,
			})
		}

		let s = match from_utf8(buf) {
			Some(s) =>
				s,
			None =>
				return Err(IoError {
					kind  : IoErrorKind::OtherIoError,
					desc  : "Tried to write invalid UTF-8",
					detail: None,
				})

		};

		for c in s.chars() {
			if self.x >= self.limit || self.x >= self.buffer.text[0].len()  as u16 {
				// Truncate everything beyond the limit
				break;
			}

			let x = self.x as uint;
			let y = self.y as uint;
			self.buffer.text[y][x] = c;

			self.x += 1;
		}

		Ok(())
	}
}
