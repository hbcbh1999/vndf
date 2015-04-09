use gfx;
use gfx_device_gl as gl;


mod buffer;
mod color;
mod renderer;
mod screen;
mod util;


pub use self::buffer::C;
pub use self::buffer::ScreenBuffer;
pub use self::color::Color;
pub use self::renderer::Renderer;
pub use self::screen::Screen;
pub use self::util::draw_border;


pub type Graphics = gfx::Graphics<gl::Device, gl::Factory>;
pub type Pos      = u16;
