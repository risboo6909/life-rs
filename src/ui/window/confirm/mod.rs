// Simple confirmation window

use super::{WindowBase, PostAction};

use piston_window::{Input, Button, Key, Context, Event, rectangle};
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

pub struct ConfirmationWindow<'a, F> {
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,

    callback: F,
}

impl<'a, F> ConfirmationWindow<'a, F> {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>,
               callback: F) -> ConfirmationWindow<'a, F> where F: FnMut(UserChoice) {

        ConfirmationWindow {
            engine: engine,
            resources: resources,

            callback: callback
        }
    }

}

impl<'a, F> WindowBase for ConfirmationWindow<'a, F> where F: FnMut(UserChoice) {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {
        rectangle([1.0, 1.0, 1.0, 0.5],
                  [0.0, 0.0, 20.0, 20.0], c.transform, g);
    }

    fn event_dispatcher(&mut self, event: &Event) -> PostAction {

        match event {

             &Event::Input(Input::Press(Button::Keyboard(Key::Y))) => {
                 (self.callback)(UserChoice::Ok);
                 PostAction::Pop
             }

            _ => PostAction::Stop

        }

    }

}
