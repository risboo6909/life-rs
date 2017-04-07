extern crate opengl_graphics;
extern crate piston_window;

mod window;

pub use self::window::confirm::ConfirmationWindow;
pub use self::window::board::{GameBoard, CellDesc, new as new_board_window};
pub use super::structs::{GameWindow, CellProp};

use super::{Resources, OPENGL};
use std::time::{Instant, Duration};
use opengl_graphics::GlGraphics;
use engine::Engine;

use std::rc::Rc;

use piston_window::{OpenGL, Context, text, clear, rectangle, line,
                    Transformed, Event, Button, Input,
                    MouseButton, Key, PistonWindow, WindowSettings, Motion};



#[derive(PartialEq)]
enum State {
    Working,
    Draw,
    Paused,
    StepByStep,
    Help,
}

pub struct UI {
    stack: Vec<Box<window::WindowBase>>,

    window: Rc<GameWindow>,
    resources: Resources,

    cur_state: State,
}

impl UI {

    pub fn push(&mut self, w: Box<window::WindowBase>) {
        self.stack.push(w);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn get_window(&self) -> Rc<GameWindow> {
        self.window.clone()
    }

//    fn draw_hud(&mut self, c: &Context, g: &mut GlGraphics) {
//        text(GREEN, 15,
//             &format!("generation {}", self.engine.cur_iteration()),
//             &mut self.resources.font,
//             c.trans(10.0, 20.0).transform, g);
//
//        text(GREEN, 15,
//             &format!("population {}", self.engine.get_board().get_population()),
//             &mut self.resources.font,
//             c.trans(150.0, 20.0).transform, g);
//
//        text(GREEN, 15,
//             &format!("update time {:.*}", 5, self.engine.get_last_iter_time()),
//             &mut self.resources.font,
//             c.trans(320.0, 20.0).transform, g);
//    }

    pub fn event_dispatcher(&mut self) {

        let mut last_iter_time = Instant::now();
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
//
//                        Event::Update(_) => {
//                            if self.cur_state == State::Working || self.cur_state == State::StepByStep {
//                                if !self.render ||
//                                    Instant::now() - last_iter_time >= Duration::from_millis(3) ||
//                                    self.cur_state == State::StepByStep {
//
//                                    self.engine.iterations(1);
//                                    last_iter_time = Instant::now();
//
//                                    if self.cur_state == State::StepByStep {
//                                        self.cur_state = State::Paused;
//                                    }
//
//                                }
//                            }
//                        }
//
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

        // hud is always visible
        //self.draw_hud(&c, g);
    }

}

pub fn new(window: Rc<GameWindow>, resources: Resources) -> UI {

    let mut ui = UI { stack: Vec::new(),
                      window: window,
                      resources: resources,
                      cur_state: State::Paused,
                    };

    let board_window = Box::new(new_board_window(ui.get_window(),
                                                 Engine::new(Some(200), Some(200))));


    ui.push(board_window);

    ui
}
