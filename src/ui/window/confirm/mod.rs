// Simple confirmation window

use super::{WindowBase, PostAction, States};

use piston_window::{Input, Button, Key, Context, Event, Transformed, rectangle, text};
use piston_window::character::CharacterCache;
use opengl_graphics::GlGraphics;

use super::super::super::engine::Engine;
use super::super::super::Resources;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq)]
pub enum UserChoice {
    Ok,
    Cancel,
}

pub struct ConfirmationWindow<'a, F>
    where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice, &mut States) {
    msg: &'a str,

    scr_width: f64,
    scr_height: f64,

    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,

    callback: F,
}

impl<'a, F> ConfirmationWindow<'a, F>
    where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice, &mut States)  {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>,
               callback: F, msg: &'a str, width: f64, height: f64) -> ConfirmationWindow<'a, F> {

        ConfirmationWindow {
            msg: msg,

            scr_width: width,
            scr_height: height,

            engine: engine,
            resources: resources,

            callback: callback
        }
    }

}

impl<'a, F> WindowBase for ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>,
    UserChoice, &mut States) {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        let font_size = 15u32;
        let msg_width = self.resources.borrow_mut().font.width(font_size, self.msg);

        rectangle([0.0, 0.0, 0.8, 1.0],
                  [80.0, 100.0, msg_width, 30.0], c.transform, g);

        text(super::GREEN, font_size,
             &format!("{}", self.msg),
             &mut self.resources.borrow_mut().font,
             c.trans(100.0, 100.0).transform, g);
    }

    fn event_dispatcher(&mut self, event: &Event, cur_state: &mut States) -> PostAction {

        match event {

             &Event::Input(Input::Press(Button::Keyboard(Key::Y))) => {
                 (self.callback)(self.engine.clone(), UserChoice::Ok, cur_state);
                 PostAction::Pop
             }

             &Event::Input(Input::Press(Button::Keyboard(Key::N))) => {
                 (self.callback)(self.engine.clone(), UserChoice::Cancel, cur_state);
                 PostAction::Pop
             }

            _ => PostAction::Stop

        }

    }

}
