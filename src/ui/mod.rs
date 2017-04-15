extern crate opengl_graphics;
extern crate piston_window;

mod window;

pub use self::window::confirm::ConfirmationWindow;

// various windows builders
use self::window::board::{GameBoard,
                          CellDesc, new as new_board_window};
use self::window::hud::{HUDWindow, new as new_hud_window};


pub use super::structs::{GraphicsWindow, CellProp};

use super::{Resources, OPENGL};
use opengl_graphics::GlGraphics;
use engine::Engine;

use std::rc::Rc;
use std::cell::RefCell;

use piston_window::{OpenGL, Context, text, clear, rectangle, line,
                    Transformed, Event, Button, Input,
                    MouseButton, Key, PistonWindow, WindowSettings, Motion};


pub struct UI<'a> {
    stack: Vec<Box<window::WindowBase + 'a>>,

    window: Rc<GraphicsWindow>,
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,
}

impl<'a> UI<'a> {

    pub fn push(&mut self, w: Box<window::WindowBase + 'a>) {
        self.stack.push(w);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn get_window(&self) -> Rc<GraphicsWindow> {
        self.window.clone()
    }

    pub fn get_engine(&self) -> Rc<RefCell<Engine<'a>>> {
        self.engine.clone()
    }

    pub fn get_resources(&self) -> Rc<RefCell<Resources>> {
        self.resources.clone()
    }

    pub fn event_dispatcher(&mut self) {

        let mut last_pos: Option<[f64; 2]> = None;

        let mut gl = GlGraphics::new(OPENGL);

        loop {

            let event = { self.window.get_window().borrow_mut().next() };

            match event {

                Some(e) => {

                    match e {

                        Event::Render(args) => {
                            gl.draw(args.viewport(), |c, g| self.paint_all(c, g));
                        }

                        _ => {
                            // update all windows one by one in order
                            for window in &mut self.stack {
                                window.event_dispatcher(&e);
                            }
                        }

                    }
                }

                None => break
            }
        }
    }

    pub fn paint_all(&mut self, c: Context, g: &mut GlGraphics) {

        // clear background
        clear([0.0, 0.0, 0.0, 1.0], g);

        // and paint all windows one by one in order
        for window in &mut self.stack {
            window.paint(c, g)
        }

    }

}

pub fn new<'a>(window: Rc<GraphicsWindow>, engine: Rc<RefCell<Engine<'a>>>, resources: Rc<RefCell<Resources>>) -> UI<'a> {

    let mut ui = UI { stack: Vec::new(),
                      window: window,
                      engine: engine,
                      resources: resources,
                    };

    let board_window = Box::new(new_board_window(ui.get_window(),
                                                 ui.get_engine()));

    let hud_window = Box::new(new_hud_window(ui.get_resources(),
                                             ui.get_engine()));

    ui.push(board_window);
    ui.push(hud_window);

    ui
}
