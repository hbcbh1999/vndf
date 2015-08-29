use glutin;

use client::render::base::Graphics;

// TODO: consider as a trait instead of an inner object?
pub struct Window {
	inner: glutin::Window,
}

impl Window {
	// TODO: window size args and make window resizable
	pub fn new(width: u32, height: u32) -> Window {
		let window = glutin::WindowBuilder::new()
			.with_title("Von Neumann Defense Force - PREVIEW VERSION".to_string())
			.with_dimensions(width, height)
			.with_vsync()
			.with_multisampling(8)
			.build_strict()
			.unwrap_or_else(|e| panic!("Error creating window: {}", e));

		unsafe { window.make_current() }
			.unwrap_or_else(|e| panic!("Error making window current: {:?}", e));
		
		Window {
			inner: window,
		}
	}

	pub fn get_size (&self) -> (u32,u32) {
		match self.inner.get_inner_size_pixels() {
			Some(size) => size,
			None       => panic!("Failed to get window size"),
		}
	}

	pub fn create_graphics(&self) -> Graphics {
		let (width,height) = self.get_size();
		Graphics::new(
			|s| self.inner.get_proc_address(s),
			(width as u16, height as u16),
		)
	}

	pub fn poll_events(&self) -> glutin::PollEventsIterator {
		self.inner.poll_events()
	}

	pub fn swap_buffers(&self) {
		self.inner
			.swap_buffers()
			.unwrap_or_else(|e| panic!("Error swapping buffers: {:?}", e)
		)
	}
}
