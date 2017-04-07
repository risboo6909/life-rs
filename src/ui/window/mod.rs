pub mod confirm;
pub mod board;

use piston_window::Context;
use opengl_graphics::GlGraphics;


pub trait WindowBase {

    fn paint(&mut self, c: Context, g: &mut GlGraphics);

}

pub trait ActiveWindow: WindowBase {

    fn event_dispatcher(&self);

}


pub trait InactiveWindow: WindowBase {


}
