/// The Gmae of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;

mod symvec;
mod board;
mod engine;

use piston_window::*;
use engine::Engine;
use board::{Board, CellDesc};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Instant, Duration};


#[derive(PartialEq)]
enum State {
    Working,
    Draw,
    Paused,
    Help,
}

struct Game {
    width: u32,
    height: u32,

    cell_width: f64,
    cell_height: f64,

    window: Rc<RefCell<PistonWindow>>,
    engine: Engine,
    cur_state: State,
}

impl Game {

    fn new(width: u32, height: u32) -> Game {

        let mut window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width, height]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut game_board = Board::new(Some(100), Some(100));

        Game {
              width: width,
              height: height,
              cell_width: 10.0,
              cell_height: 10.0,

              window: Rc::new(RefCell::new(window)),
              engine: Engine::new(game_board),
              cur_state: State::Paused
            }

    }

    fn event_dispatcher(&mut self) {

        let mut last_iter_time = Instant::now();
        let mut last_pos: Option<[f64; 2]> = None;

        loop {

            let event = { self.window.borrow_mut().next() };

            match event {

                Some(e) => {

                    if let Event::Render(_) = e {
                        self.paint(&e);
                        if self.cur_state == State::Working {
                            if Instant::now() - last_iter_time >= Duration::from_millis(5) {
                                self.engine.one_iteration();
                                last_iter_time = Instant::now();
                            }
                        }
                    }

                    if let Some(Button::Keyboard(Key::P)) = e.press_args() {
                        if self.cur_state == State::Working {
                            self.cur_state = State::Paused;
                        } else {
                            self.cur_state = State::Working;
                        }
                    }

                    if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                        println!("right");
                    }

                    if let Some(Button::Keyboard(Key::NumPadMinus)) = e.press_args() {
                        println!("zoom out");
                    }

                    if let Some(Button::Keyboard(Key::NumPadPlus)) = e.press_args() {
                        println!("zoom in");
                    }

                    if let Some(button) = e.press_args() {
                        if button == Button::Mouse(MouseButton::Left) {
                            self.cur_state = State::Draw;
                        }
                    }

                    if let Some(button) = e.release_args() {
                        if button == Button::Mouse(MouseButton::Left) {
                            if last_pos.is_some() {
                                let pos = last_pos.unwrap();
                                self.born_at(pos[0] as f64, pos[1] as f64);
                                self.cur_state = State::Paused;
                            }
                        }
                    }

                    if let Some(pos) = e.mouse_cursor_args() {
                        if self.cur_state == State::Draw {
                            self.born_at(pos[0] as f64, pos[1] as f64);
                        } else {
                            last_pos = Some(pos);
                        }
                    }

                }

                None => break

            }
        }
    }

    fn born_at(&mut self, x: f64, y: f64) {
        let (col, row) = self.to_logical(x, y);
        let board = self.engine.get_board_mut();
        board.born_at(col, row);
    }

    fn to_screen(&self, col: isize, row: isize) -> (f64, f64) {
         (col as f64 * self.cell_width + (self.width as f64 / 2.0),
          row as f64 * self.cell_height + (self.height as f64 / 2.0))
    }

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize) {
        let col = ((x - (self.width as f64) / 2.0) / self.cell_width) as isize;
        let row = ((y - (self.height as f64) / 2.0) / self.cell_height) as isize;
        (col, row)
    }

    fn paint(&self, e: &Event) {

            self.window.borrow_mut().draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let board = self.engine.get_board();

            for CellDesc {coord, is_alive, ..} in board.into_iter() {

                if is_alive {

                    let col = coord.col;
                    let row = coord.row;

                    let (x, y) = self.to_screen(col, row);
                    //println!("{}, {}", x, y);
                    rectangle([0.5, 1.0, 0.0, 0.3],
                              [x, y, 10.0, 10.0],
                               c.transform, g);
                }
            }

        });

    }

}

fn main() {

    let mut game = Game::new(1024, 768);
    game.event_dispatcher();

}
