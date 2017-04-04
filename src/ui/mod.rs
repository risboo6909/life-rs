mod window;

pub use self::window::confirm::ConfirmationWindow;
pub use self::window::board::{GameBoard, new as new_board_window};
pub use super::structs::GameWindow;

pub struct UI {
    stack: Vec<Box<window::WindowBase>>,
    window: GameWindow,
}

impl UI {

    pub fn push(&mut self, w: Box<window::WindowBase>) {
        self.stack.push(w);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

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
                            gl.draw(args.viewport(), |c, g| self.paint(c, g));
                        }

                        Event::Update(_) => {
                            if self.cur_state == State::Working || self.cur_state == State::StepByStep {
                                if !self.render ||
                                    Instant::now() - last_iter_time >= Duration::from_millis(3) ||
                                    self.cur_state == State::StepByStep {

                                    self.engine.iterations(1);
                                    last_iter_time = Instant::now();

                                    if self.cur_state == State::StepByStep {
                                        self.cur_state = State::Paused;
                                    }

                                }
                            }
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::P))) => {
                            // pause/unpause
                            if self.cur_state == State::Working {
                                self.cur_state = State::Paused;
                                // always enable rendering in pause mode
                                self.render = true;
                            } else {
                                self.cur_state = State::Working;
                            }
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::S))) => {
                            // step by step mode
                            if self.cur_state == State::Working || self.cur_state == State::Paused {
                                self.cur_state = State::StepByStep;
                                // always enable rendering in step by step mode
                                self.render = true;
                            }
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::G))) => {
                            // show/hide grid
                            self.show_grid = !self.show_grid;
                        }


                        Event::Input(Input::Press(Button::Mouse(MouseButton::Left))) => {
                            self.cur_state = State::Draw;
                        }

                        Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
                            if last_pos.is_some() {
                                let pos = last_pos.unwrap();
                                self.born_or_kill(true, pos[0], pos[1]);

                                self.cur_state = State::Paused;
                            }
                        }

                        Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
                            if self.cur_state == State::Draw {
                                self.born_or_kill(false, x, y);
                            }
                            last_pos = Some([x, y]);
                        }

                        // movements control ->
                        Event::Input(Input::Press(Button::Keyboard(Key::Right))) => {
                            self.cam.move_right();
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Right))) => {
                            self.cam.reset_move_step();
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Left))) => {
                            self.cam.move_left();
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Left))) => {
                            self.cam.reset_move_step();
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Up))) => {
                            self.cam.move_up();
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Up))) => {
                            self.cam.reset_move_step();
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Down))) => {
                            self.cam.move_down();;
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Down))) => {
                            self.cam.reset_move_step();
                        }
                        // movements control <-

                        // zoom out ->
                        Event::Input(Input::Press(Button::Keyboard(Key::NumPadMinus))) => {
                            self.cam.zoom_out();
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Minus))) => {
                            self.cam.zoom_out();
                        }
                        // zoom out <-

                        // zoom in ->
                        Event::Input(Input::Press(Button::Keyboard(Key::NumPadPlus))) => {
                            self.cam.zoom_in();
                        }

                        // use "Equals" instead of "Plus" to avoid holding shift key requirement
                        Event::Input(Input::Press(Button::Keyboard(Key::Equals))) => {
                            self.cam.zoom_in();
                        }
                        // zoom in <-

                        Event::Input(Input::Press(Button::Keyboard(Key::R))) => {
                            // If in pause mode - fill board with a random pattern
                            if self.cur_state == State::Paused {
                                let board = self.engine.create_random(0.3);
                                self.engine.set_board(board);
                            }
                            else {
                                // otherwise enable/disable rendering
                                self.render = !self.render;
                            }

                        }

                        _ => {}
                    }
                }

                None => break
            }
        }
    }

    pub fn paint_all(&self) {

    }

}

pub fn new(window: GameWindow) -> UI {
    UI { stack: Vec::new(),
         window: window }
}
