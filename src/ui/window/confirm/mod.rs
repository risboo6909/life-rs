// Simple confirmation window

use super::WindowBase;

use piston_window::{Context, Event};
use opengl_graphics::GlGraphics;


pub struct ConfirmationWindow {
    state: isize,
}

impl WindowBase for ConfirmationWindow {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

    }

    fn event_dispatcher(&mut self, event: &Event) {

    }

}
