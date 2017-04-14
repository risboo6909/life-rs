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
                            for window in &mut self.stack {
                                window.event_dispatcher(&e);
                            }
                        }

//                        Event::Input(Input::Press(Button::Keyboard(Key::P))) => {
//                            // pause/unpause
//                            if self.cur_state == State::Working {
//                                self.cur_state = State::Paused;
//                                // always enable rendering in pause mode
//                                self.render = true;
//                            } else {
//                                self.cur_state = State::Working;
//                            }
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::S))) => {
//                            // step by step mode
//                            if self.cur_state == State::Working || self.cur_state == State::Paused {
//                                self.cur_state = State::StepByStep;
//                                // always enable rendering in step by step mode
//                                self.render = true;
//                            }
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::G))) => {
//                            // show/hide grid
//                            self.show_grid = !self.show_grid;
//                        }
//
//
//                        Event::Input(Input::Press(Button::Mouse(MouseButton::Left))) => {
//                            self.cur_state = State::Draw;
//                        }
//
//                        Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
//                            if last_pos.is_some() {
//                                let pos = last_pos.unwrap();
//                                self.born_or_kill(true, pos[0], pos[1]);
//
//                                self.cur_state = State::Paused;
//                            }
//                        }
//
//                        Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
//                            if self.cur_state == State::Draw {
//                                self.born_or_kill(false, x, y);
//                            }
//                            last_pos = Some([x, y]);
//                        }
//
//                        // movements control ->
//                        Event::Input(Input::Press(Button::Keyboard(Key::Right))) => {
//                            self.cam.move_right();
//                        }
//
//                        Event::Input(Input::Release(Button::Keyboard(Key::Right))) => {
//                            self.cam.reset_move_step();
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::Left))) => {
//                            self.cam.move_left();
//                        }
//
//                        Event::Input(Input::Release(Button::Keyboard(Key::Left))) => {
//                            self.cam.reset_move_step();
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::Up))) => {
//                            self.cam.move_up();
//                        }
//
//                        Event::Input(Input::Release(Button::Keyboard(Key::Up))) => {
//                            self.cam.reset_move_step();
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::Down))) => {
//                            self.cam.move_down();;
//                        }
//
//                        Event::Input(Input::Release(Button::Keyboard(Key::Down))) => {
//                            self.cam.reset_move_step();
//                        }
//                        // movements control <-
//
//                        // zoom out ->
//                        Event::Input(Input::Press(Button::Keyboard(Key::NumPadMinus))) => {
//                            self.cam.zoom_out();
//                        }
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::Minus))) => {
//                            self.cam.zoom_out();
//                        }
//                        // zoom out <-
//
//                        // zoom in ->
//                        Event::Input(Input::Press(Button::Keyboard(Key::NumPadPlus))) => {
//                            self.cam.zoom_in();
//                        }
//
//                        // use "Equals" instead of "Plus" to avoid holding shift key requirement
//                        Event::Input(Input::Press(Button::Keyboard(Key::Equals))) => {
//                            self.cam.zoom_in();
//                        }
//                        // zoom in <-
//
//                        Event::Input(Input::Press(Button::Keyboard(Key::R))) => {
//                            // If in pause mode - fill board with a random pattern
//                            if self.cur_state == State::Paused {
//                                let board = self.engine.create_random(0.3);
//                                self.engine.set_board(board);
//                            }
//                            else {
//                                // otherwise enable/disable rendering
//                                self.render = !self.render;
//                            }
//
//                        }

                        _ => {}
                    }
                }

                None => break
            }
        }
    }

    pub fn paint_all(&mut self, c: Context, g: &mut GlGraphics) {

        clear([0.0, 0.0, 0.0, 1.0], g);

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
