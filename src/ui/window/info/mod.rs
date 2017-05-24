// Simple info window

use super::{WindowBase, InfoWindowTrait, PostAction, States};

use piston_window::{Input, Button, Key, Context, Event};
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

impl<'a> InfoWindowTrait for InfoWindow<'a> {

}

impl<'a> WindowBase for InfoWindow<'a> {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        let (scr_width, scr_height) = (self.scr_width, self.scr_height);
        let resources = self.resources.clone();

        self.paint_info_window(c, g, scr_width, scr_height,
                               resources, self.msg, "press Enter to continue");
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
