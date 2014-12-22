use std::comm::TryRecvError;
use std::io::{
	stdin,
	IoResult,
};

use client::platform::{
	Frame,
	Input,
};

use self::input::{
	InputReader,
	ReadInput,
};
use self::render::{
	HeadlessRenderer,
	Render,
	Renderer,
};


pub mod input;
pub mod render;


// TODO: Merge ReadInput and Render into PlatformIo
pub trait PlatformIo: ReadInput + Render {
	fn new() -> IoResult<Self>;
}


pub struct PlayerIo {
	input_reader: InputReader,
	renderer    : Renderer,
}

impl PlatformIo for PlayerIo {
	fn new() -> IoResult<PlayerIo> {
		let input_reader = InputReader::new();
		let renderer = match Renderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => return Err(error),
		};

		Ok(PlayerIo {
			input_reader: input_reader,
			renderer    : renderer,
		})
	}
}

impl ReadInput for PlayerIo {
	fn input(&mut self) -> Input {
		self.input_reader.input()
	}
}

impl Render for PlayerIo {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame)
	}
}


// TODO: Inline HeadlessInputRender and HeadlessRenderer
pub struct HeadlessIo {
	last_input: Input,
	receiver  : Receiver<Input>,
	renderer  : HeadlessRenderer,
}

impl PlatformIo for HeadlessIo {
	fn new() -> IoResult<HeadlessIo> {
		let (sender, receiver) = channel();

		spawn(move || {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) => match Input::from_json(line.as_slice()) {
						Ok(input) =>
							sender.send(input),
						Err(error) =>
							panic!("Error decoding input: {}\n", error),
					},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		let renderer = match HeadlessRenderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => return Err(error),
		};

		Ok(HeadlessIo {
			receiver  : receiver,
			last_input: Input::new(),
			renderer  : renderer,
		})
	}
}

impl ReadInput for HeadlessIo {
	fn input(&mut self) -> Input {
		match self.receiver.try_recv() {
			Ok(input) => {
				self.last_input = input.clone();
				input
			},
			Err(error) => match error {
				TryRecvError::Empty        => self.last_input.clone(),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

impl Render for HeadlessIo {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame)
	}
}