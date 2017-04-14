pub mod confirm;
pub mod board;
pub mod hud;

use opengl_graphics::GlGraphics;

pub use piston_window::{Context, line, rectangle, text, Event};

pub const GREEN: [f32; 4] = [0.5, 1.0, 0.0, 1.0];
pub const GRAY: [f32; 4] = [100.0, 100.0, 100.0, 1.0];
pub const RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];


pub trait WindowBase {

    fn paint(&mut self, c: Context, g: &mut GlGraphics);
    fn event_dispatcher(&mut self, event: &Event);

}
