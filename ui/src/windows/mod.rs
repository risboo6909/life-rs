pub mod confirm;
pub mod board;
pub mod hud;
pub mod info;

use opengl_graphics::GlGraphics;
use std::cell::Cell;

pub use piston_window::{Context, Event, Transformed, line, rectangle, text};
use piston_window::character::CharacterCache;

use super::Resources;
use super::cam::Cam;

use std::rc::Rc;
use std::cell::RefCell;

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

    fn paint(&mut self, c: Context, g: &mut GlGraphics);
    fn event_dispatcher(&mut self, event: &Event, cur_state: &Cell<States>) -> PostAction;
    fn is_modal(&self) -> bool { false }

}

pub trait InfoWindowTrait: WindowBase {

    fn paint_info_window(&mut self, c: Context, g: &mut GlGraphics,
                         scr_width: f64, scr_height: f64, resources: Rc<RefCell<Resources>>,
                         msg: &str, prompt: &str) {

        let font_size = 15u32;

        let msg_width = resources.borrow_mut().font.width(font_size, msg);
        let prompt_width = resources.borrow_mut().font.width(font_size, prompt);

        let prompt_outer_window_width = msg_width + 60.0;
        let prompt_outer_window_height = 60.0;

        let prompt_window_offset_x =  0.5 * (scr_width - prompt_outer_window_width);
        let prompt_window_offset_y =  0.5 * (scr_height - prompt_outer_window_height);

        let msg_offset_x = prompt_window_offset_x + 0.5 * (prompt_outer_window_width - msg_width);
        let msg_offset_y = prompt_window_offset_y + 10.0 + font_size as f64;

        let prompt_offset_x = msg_offset_x + 0.5 * (msg_width - prompt_width);

        rectangle([0.4, 0.4, 0.0, 1.0],
                  [prompt_window_offset_x, prompt_window_offset_y, prompt_outer_window_width,
                      prompt_outer_window_height], c.transform, g);

        rectangle([0.0, 0.0, 0.8, 1.0],
                  [prompt_window_offset_x + 10.0, prompt_window_offset_y + 10.0, prompt_outer_window_width - 20.0,
                      prompt_outer_window_height - 20.0], c.transform, g);

        text(WHITE, font_size,
             &format!("{}", msg),
             &mut resources.borrow_mut().font,
             c.trans(msg_offset_x, msg_offset_y).transform, g);

        text(GREEN, font_size,
             &prompt,
             &mut resources.borrow_mut().font,
             c.trans(prompt_offset_x, msg_offset_y + 20.0).transform, g);

    }

}
