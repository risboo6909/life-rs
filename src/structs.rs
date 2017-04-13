use ::cam::Cam;
use piston_window::PistonWindow;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CellProp {
    cell_width: f64,
    cell_height: f64,
}

impl CellProp {
    pub fn new(cell_width: f64, cell_height: f64) -> Self {
        CellProp { cell_width: cell_width, cell_height: cell_height }
    }

    #[inline]
    pub fn get_width(&self, cam: &Cam) -> f64 {
        self.cell_width * cam.get_scale()
    }

    #[inline]
    pub fn get_height(&self, cam: &Cam) -> f64 {
        self.cell_height * cam.get_scale()
    }

    #[inline]
    pub fn get_half_width(&self, cam: &Cam) -> f64 {
        0.5 * self.get_width(&cam)
    }

    #[inline]
    pub fn get_half_height(&self, cam: &Cam) -> f64 {
        0.5 * self.get_height(&cam)
    }
}


pub struct GraphicsWindow {

    window: Rc<RefCell<PistonWindow>>,

    width: f64,
    height: f64,
}


impl GraphicsWindow {

    pub fn new(window_width: f64, window_height: f64, window: PistonWindow) -> Self {
        GraphicsWindow { width: window_width,
                         height: window_height,
                         window: Rc::new(RefCell::new(window)) }
    }

    #[inline]
    pub fn get_width(&self) -> f64 {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> f64 {
        self.height
    }

    #[inline]
    pub fn get_half_width(&self) -> f64 {
        0.5 * self.get_width()
    }

    #[inline]
    pub fn get_half_height(&self) -> f64 {
        0.5 * self.get_height()
    }

    #[inline]
    pub fn get_window(&self) -> &Rc<RefCell<PistonWindow>> {
        &self.window
    }

}
