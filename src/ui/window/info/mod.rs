// Simple info window

use super::{WindowBase, PostAction, States};

use piston_window::{Input, Button, Key, Context, Event, Transformed, rectangle, text};
use piston_window::character::CharacterCache;
use opengl_graphics::GlGraphics;

use super::super::super::engine::Engine;
use super::super::super::Resources;

use std::rc::Rc;
use std::cell::{RefCell, Cell};


pub struct InfoWindow<'a> {

    msg: &'a str,

    scr_width: f64,
    scr_height: f64,

    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,

}

impl<'a> InfoWindow<'a> {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>,
               msg: &'a str, width: f64, height: f64) -> Self {

        InfoWindow {
            msg: msg,

            scr_width: width,
            scr_height: height,

            engine: engine,
            resources: resources

        }
    }

}

impl<'a> WindowBase for InfoWindow<'a> {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        let prompt = "press Enter to continue";

        let font_size = 15u32;

        let msg_width = self.resources.borrow_mut().font.width(font_size, self.msg);
        let prompt_width = self.resources.borrow_mut().font.width(font_size, prompt);

        let prompt_outer_window_width = msg_width + 60.0;
        let prompt_outer_window_height = 60.0;

        let prompt_window_offset_x =  0.5 * (self.scr_width - prompt_outer_window_width);
        let prompt_window_offset_y =  0.5 * (self.scr_height - prompt_outer_window_height);

        let msg_offset_x = prompt_window_offset_x + 0.5 * (prompt_outer_window_width - msg_width);
        let msg_offset_y = prompt_window_offset_y + 10.0 + font_size as f64;

        let prompt_offset_x = msg_offset_x + 0.5 * (msg_width - prompt_width);

        rectangle([0.4, 0.4, 0.0, 1.0],
                  [prompt_window_offset_x, prompt_window_offset_y, prompt_outer_window_width,
                      prompt_outer_window_height], c.transform, g);

        rectangle([0.0, 0.0, 0.8, 1.0],
                  [prompt_window_offset_x + 10.0, prompt_window_offset_y + 10.0, prompt_outer_window_width - 20.0,
                      prompt_outer_window_height - 20.0], c.transform, g);

        text(super::WHITE, font_size,
             &format!("{}", self.msg),
             &mut self.resources.borrow_mut().font,
             c.trans(msg_offset_x, msg_offset_y).transform, g);

        text(super::GREEN, font_size,
             &prompt,
             &mut self.resources.borrow_mut().font,
             c.trans(prompt_offset_x, msg_offset_y + 20.0).transform, g);

    }

    fn event_dispatcher(&mut self, event: &Event, cur_state: &Cell<States>) -> PostAction {

        match event {

             &Event::Input(Input::Press(Button::Keyboard(Key::Return))) => {
                 PostAction::Pop
             },

            _ => PostAction::Stop

        }

    }

    fn is_modal(&self) -> bool {
        true
    }

}
