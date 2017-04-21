// Simple confirmation window

use super::{WindowBase, PostAction};

use piston_window::{Input, Button, Key, Context, Event, rectangle};
use opengl_graphics::GlGraphics;

use super::super::super::engine::Engine;
use super::super::super::Resources;

use std::rc::Rc;
use std::cell::RefCell;

//enum States {
//
//}

pub struct ConfirmationWindow<'a> {
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>
}

impl<'a> ConfirmationWindow<'a> {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>) ->
                                                                    ConfirmationWindow<'a> {
        ConfirmationWindow {
            engine: engine,
            resources: resources,
        }
    }

}

impl<'a> WindowBase for ConfirmationWindow<'a> {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {
        rectangle([1.0, 1.0, 1.0, 0.5],
                  [0.0, 0.0, 20.0, 20.0], c.transform, g);
    }

    fn event_dispatcher(&mut self, event: &Event) -> PostAction {

        match event {

             &Event::Input(Input::Press(Button::Keyboard(Key::Y))) => {
                 PostAction::Pop
             }

            _ => PostAction::Stop

        }

    }

}
