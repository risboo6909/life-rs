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

struct Cam {

    x: f64,
    y: f64,

    scale: f64,

}

impl Cam {

    fn new(x: f64, y: f64, scale: f64) -> Self {
        Cam { x: x, y: y, scale: scale }
    }

    fn translate(&self, x: f64, y: f64) -> (f64, f64) {
        (x + self.x, y + self.y)
    }

    fn scale(&self, width: f64, height: f64) -> (f64, f64) {
        (self.scale * width, self.scale * height)
    }

    fn zoom_out(&mut self, k: f64) {
        self.scale -= k;
    }

    fn zoom_in(&mut self, k: f64) {
        self.scale += k;
    }

    fn move_right(&mut self, offset: f64) {
        self.x -= offset;
    }

    fn move_left(&mut self, offset: f64) {
        self.x += offset;
    }

    fn move_up(&mut self, offset: f64) {
        self.y += offset;
    }

    fn move_down(&mut self, offset: f64) {
        self.y -= offset;
    }

}

struct Game {

    width: u32,
    height: u32,

    cell_width: f64,
    cell_height: f64,

    move_step: f64,

    acceleration: f64,

    window: Rc<RefCell<PistonWindow>>,
    engine: Engine,
    cam: Cam,
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

        let mut game_board = Board::new(Some(200), Some(200));

        Game {
              width: width,
              height: height,
              cell_width: 10.0,
              cell_height: 10.0,

              acceleration: 1.4,
              move_step: 1.0,

              window: Rc::new(RefCell::new(window)),
              engine: Engine::new(game_board),
              cam: Cam::new(0.0, 0.0, 1.0),
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
                            if Instant::now() - last_iter_time >= Duration::from_millis(50) {
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

                    else if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                        self.cam.move_right(self.move_step);
                        self.move_step *= self.acceleration;
                    }

                    else if let Some(Button::Keyboard(Key::Right)) = e.release_args() {
                        self.move_step = 1.0
                    }

                    else if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
                        self.cam.move_left(self.move_step);
                        self.move_step *= self.acceleration;
                    }

                    else if let Some(Button::Keyboard(Key::Left)) = e.release_args() {
                        self.move_step = 1.0
                    }

                    else if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
                        self.cam.move_up(self.move_step);
                        self.move_step *= self.acceleration;
                    }

                    else if let Some(Button::Keyboard(Key::Up)) = e.release_args() {
                        self.move_step = 1.0
                    }

                    else if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
                        self.cam.move_down(self.move_step);
                        self.move_step *= self.acceleration;
                    }

                    else if let Some(Button::Keyboard(Key::Down)) = e.release_args() {
                        self.move_step = 1.0
                    }

                    else if let Some(Button::Keyboard(Key::NumPadMinus)) = e.press_args() {
                        self.cam.zoom_out(0.1);
                    }

                    else if let Some(Button::Keyboard(Key::NumPadPlus)) = e.press_args() {
                        self.cam.zoom_in(self.move_step);
                    }

                    else if let Some(button) = e.press_args() {
                        if button == Button::Mouse(MouseButton::Left) {
                            self.cur_state = State::Draw;
                        }
                    }

                    else if let Some(button) = e.release_args() {
                        if button == Button::Mouse(MouseButton::Left) {
                            if last_pos.is_some() {
                                let pos = last_pos.unwrap();
                                self.born_at(pos[0], pos[1]);
                                self.cur_state = State::Paused;
                            }
                        }
                    }

                    else if let Some(pos) = e.mouse_cursor_args() {
                        if self.cur_state == State::Draw {
                            self.born_at(pos[0], pos[1]);
                        }
                        last_pos = Some(pos);
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
        let (cell_width, cell_height) = self.cam.scale(self.cell_width,
                                                       self.cell_height);
        let x = col as f64 * cell_width + (0.5 * self.width as f64) - 5.0;
        let y = row as f64 * cell_height + (0.5 * self.height as f64) - 5.0;
        self.cam.translate(x, y)
    }

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize) {
        let mut offset_x = x - 0.5 * (self.width as f64);
        let mut offset_y = y - 0.5 * (self.height as f64);

        if offset_x < 0.0 {
            offset_x -= 5.0;
        }
        if offset_x > 0.0 {
            offset_x += 5.0;
        }

        if offset_y < 0.0 {
            offset_y -= 5.0;
        }
        if offset_y > 0.0 {
            offset_y += 5.0;
        }

        let col = (offset_x / self.cell_width) as isize;
        let row = (offset_y / self.cell_height) as isize;
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

                    let (cell_width, cell_height) = self.cam.scale(self.cell_width,
                                                                   self.cell_height);

                    rectangle([0.5, 1.0, 0.0, 0.3],
                              [x, y, cell_width, cell_height],
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
