// HUD window

use super::{ActiveWindow, WindowBase, text};

use piston_window::Context;
use opengl_graphics::GlGraphics;


pub struct HUDWindow {
    //state: isize,
}

impl WindowBase for HUDWindow {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

//        text(super::GREEN, 15,
//             &format!("generation {}", self.engine.cur_iteration()),
//             &mut self.resources.font,
//             c.trans(10.0, 20.0).transform, g);
//
//        text(super::GREEN, 15,
//             &format!("population {}", self.engine.get_board().get_population()),
//             &mut self.resources.font,
//             c.trans(150.0, 20.0).transform, g);
//
//        text(super::GREEN, 15,
//             &format!("update time {:.*}", 5, self.engine.get_last_iter_time()),
//             &mut self.resources.font,
//             c.trans(320.0, 20.0).transform, g);

    }

}

impl ActiveWindow for HUDWindow {

    fn event_dispatcher(&self) {

    }

}


pub fn new() -> HUDWindow {

    HUDWindow {}

}