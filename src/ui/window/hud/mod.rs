// HUD window
extern crate piston_window;


use piston_window::{Context, Transformed, text};

use super::{ActiveWindow, WindowBase};
use super::super::super::engine::Engine;
use super::super::super::Resources;

use opengl_graphics::GlGraphics;

use std::rc::Rc;
use std::cell::RefCell;


pub struct HUDWindow<'a> {
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>
    //state: isize,
}

impl<'a> WindowBase for HUDWindow<'a> {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        text(super::GREEN, 15,
             &format!("generation {}", self.engine.borrow().cur_iteration()),
             &mut self.resources.borrow_mut().font,
             c.trans(10.0, 20.0).transform, g);

        text(super::GREEN, 15,
             &format!("population {}", self.engine.borrow().get_board().get_population()),
             &mut self.resources.borrow_mut().font,
             c.trans(150.0, 20.0).transform, g);

        text(super::GREEN, 15,
             &format!("update time {:.*}", 5, self.engine.borrow().get_last_iter_time()),
             &mut self.resources.borrow_mut().font,
             c.trans(320.0, 20.0).transform, g);

    }

}

impl<'a> ActiveWindow for HUDWindow<'a> {

    fn event_dispatcher(&self) {

    }

}


pub fn new<'a>(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>) -> HUDWindow<'a> {

    HUDWindow {
        resources: resources,
        engine: engine
    }

}