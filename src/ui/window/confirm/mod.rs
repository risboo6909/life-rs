// Simple confirmation window

use super::{ActiveWindow, WindowBase};

use piston_window::Context;
use opengl_graphics::GlGraphics;


pub struct ConfirmationWindow {
    state: isize,
}

impl WindowBase for ConfirmationWindow {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

    }

}

impl ActiveWindow for ConfirmationWindow {

    fn event_dispatcher(&self) {

    }

}
