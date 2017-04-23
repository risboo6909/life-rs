// Simple confirmation window

use super::{WindowBase, PostAction, States};

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

pub struct ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice, &mut States) {
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,

    callback: F,
}

impl<'a, F> ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice, &mut States)  {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>,
               callback: F) -> ConfirmationWindow<'a, F> {

        ConfirmationWindow {
            engine: engine,
            resources: resources,

            callback: callback
        }
    }

}

impl<'a, F> WindowBase for ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice, &mut States) {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {
        rectangle([1.0, 1.0, 1.0, 0.5],
                  [0.0, 0.0, 20.0, 20.0], c.transform, g);
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
