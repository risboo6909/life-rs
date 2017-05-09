pub mod confirm;
pub mod board;
pub mod hud;
pub mod info;

use opengl_graphics::GlGraphics;
use std::cell::Cell;

pub use piston_window::{Context, line, rectangle, text, Event};

pub const GREEN: [f32; 4] = [0.5, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const GRAY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


pub enum PostAction {
    Transfer,
    Stop,
    Pop,
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum States {
    Working,
    Draw,
    Paused,
    StepByStep,
    Help,
}

pub trait WindowBase {

    fn paint(&mut self, Context, &mut GlGraphics);
    fn event_dispatcher(&mut self, &Event, &Cell<States>) -> PostAction;
    fn is_modal(&self) -> bool { false }

}
