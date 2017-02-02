/// The Gmae of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;

mod symvec;
mod board;
mod engine;

use find_folder::Search;
use piston_window::{OpenGL, Context, text, clear, rectangle, line,
                    Transformed, Event, Button, Input,
                    MouseButton, Key, MouseCursorEvent, ReleaseEvent,
                    PressEvent, PistonWindow, WindowSettings, Motion};

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use engine::Engine;
use board::{Board, CellDesc};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Instant, Duration};

const OPENGL: piston_window::OpenGL = OpenGL::V3_2;


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


struct Resources {
    font: GlyphCache<'static>
}


impl Cam {

    fn new(x: f64, y: f64, scale: f64) -> Self {
        Cam { x: x, y: y, scale: scale }
    }

    fn translate(&self, x: f64, y: f64) -> (f64, f64) {
        (x + self.x, y + self.y)
    }

    fn translate_inv(&self, x: f64, y: f64) -> (f64, f64) {
        (x - self.x, y - self.y)
    }

    fn scale(&self, width: f64, height: f64) -> (f64, f64) {
        (self.scale * width, self.scale * height)
    }

    fn scale_inv(&self, width: f64, height: f64) -> (f64, f64) {
        ((1.0/self.scale) * width, (1.0/self.scale) * height)
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

    width: f64,
    height: f64,

    cell_width: f64,
    cell_height: f64,

    move_step: f64,
    acceleration: f64,

    show_grid: bool,

    window: Rc<RefCell<PistonWindow>>,
    engine: Engine,
    cam: Cam,
    cur_state: State,

    resources: Resources,
}

impl Game {

    fn new(width: f64, height: f64) -> Game {

        let mut window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width as u32, height as u32]
        ).opengl(OPENGL)
         .samples(8)
         .exit_on_esc(true)
         .build()
         .unwrap();

        //window.set_ups(60);
        //window.set_max_fps(60);

        let mut game_board = Board::new(Some(200), Some(200));

        Game {

                // window width and height in pixels
                width: width,
                height: height,

                // cell and width of a cell in pixels
                cell_width: 10.0,
                cell_height: 10.0,

                acceleration: 1.4,
                move_step: 1.0,

                show_grid: true,

                window: Rc::new(RefCell::new(window)),
                engine: Engine::new(game_board),
                cam: Cam::new(0.0, 0.0, 1.0),
                cur_state: State::Paused,

                resources: Resources {
                    font: GlyphCache::new(Search::ParentsThenKids(3, 3).
                                          for_folder("assets").unwrap().
                                          join("Roboto-Regular.ttf")).unwrap(),
                },

            }

    }

    fn event_dispatcher(&mut self) {

        let mut last_iter_time = Instant::now();
        let mut last_pos: Option<[f64; 2]> = None;

        let mut gl = GlGraphics::new(OPENGL);

        loop {

            let event = { self.window.borrow_mut().next() };

            match event {

                Some(e) => {

                    match e {

                        Event::Render(args) => {
                            gl.draw(args.viewport(), |c, g| self.paint(c, g));
                        }

                        Event::Update(args) => {
                            if self.cur_state == State::Working {
                                if Instant::now() - last_iter_time >= Duration::from_millis(10) {
                                    self.engine.one_iteration();
                                    last_iter_time = Instant::now();
                                }
                            }
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::P))) => {
                            // pause/unpause
                            if self.cur_state == State::Working {
                                self.cur_state = State::Paused;
                            } else {
                                self.cur_state = State::Working;
                            }
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::G))) => {
                            // show/hide grid
                            self.show_grid = !self.show_grid;
                        }

                        Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
                            if last_pos.is_some() {
                                let pos = last_pos.unwrap();
                                let (t_x, t_y) = self.cam.translate_inv(pos[0], pos[1]);
                                self.born_or_kill(true, t_x, t_y);
                                self.cur_state = State::Paused;
                            }
                        }

                        Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
                            if self.cur_state == State::Draw {
                                let (t_x, t_y) = self.cam.translate_inv(x, y);
                                self.born_or_kill(false, t_x, t_y);
                            }
                            last_pos = Some([x, y]);
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Right))) => {
                            self.cam.move_right(self.move_step);
                            self.move_step *= self.acceleration;
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Right))) => {
                            self.move_step = 1.0;
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Left))) => {
                            self.cam.move_left(self.move_step);
                            self.move_step *= self.acceleration;
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Left))) => {
                            self.move_step = 1.0;
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Up))) => {
                            self.cam.move_up(self.move_step);
                            self.move_step *= self.acceleration;
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Up))) => {
                            self.move_step = 1.0;
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Down))) => {
                            self.cam.move_down(self.move_step);
                            self.move_step *= self.acceleration;
                        }

                        Event::Input(Input::Release(Button::Keyboard(Key::Down))) => {
                            self.move_step = 1.0;
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::NumPadMinus))) => {
                            self.cam.zoom_out(0.1);
                        }

                        Event::Input(Input::Press(Button::Keyboard(Key::Z))) => {
                            self.cam.zoom_in(self.move_step);
                        }

                        Event::Input(Input::Press(Button::Mouse(MouseButton::Left))) => {
                            self.cur_state = State::Draw;
                        }

                        _ => {}

                    }

                }

                None => break

            }

        }
    }

    fn born_or_kill(&mut self, kill_alive: bool, x: f64, y: f64) {
        let (col, row) = self.to_logical(x, y);
        let board = self.engine.get_board_mut();

        if kill_alive && board.is_alive(col, row) {
            board.kill_at(col, row)
        } else {
            board.born_at(col, row);
        }
    }

    fn to_screen(&self, col: isize, row: isize) -> (f64, f64) {

        let (cell_width, cell_height) = self.cam.scale(self.cell_width,
                                                       self.cell_height);

        let half_cell_width = 0.5 * cell_width;
        let half_cell_height = 0.5 * cell_height;

        let x = col as f64 * cell_width + (0.5 * self.width) - half_cell_width;
        let y = row as f64 * cell_height + (0.5 * self.height) - half_cell_height;

        self.cam.translate(x, y)

    }

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize) {

        let (cell_width, cell_height) = self.cam.scale(self.cell_width,
                                                       self.cell_height);

        let mut offset_x = x - 0.5 * self.width;
        let mut offset_y = y - 0.5 * self.height;

        if offset_x < 0.0 {
            offset_x -= 5.0;
        } else if offset_x > 0.0 {
            offset_x += 5.0;
        }

        if offset_y < 0.0 {
            offset_y -= 5.0;
        } else if offset_y > 0.0 {
            offset_y += 5.0;
        }


        let col = (offset_x / cell_width) as isize;
        let row = (offset_y / cell_height) as isize;

        (col, row)

    }

    fn draw_grid(&self, c: &Context, g: &mut GlGraphics) {

        let (grid_width, grid_height) = self.cam.scale(self.cell_width,
                                                       self.cell_height);

        let offset_x = (grid_width - (0.5 * self.width % grid_width)) - 0.5 * grid_width;
        let offset_y = (grid_height - (0.5 * self.height % grid_height)) - 0.5 * grid_height;

        let mut x = self.cam.x - offset_x;
        let mut y = self.cam.y - offset_y;

        // horizontal lines
        while y < self.height as f64 {

            line([100.0, 100.0, 100.0, 1.0], 0.09,
                 [0.0, y, self.width, y],
                  c.transform, g);

            y += grid_height;

        }

        // vertical lines
        while x < self.width as f64 {

            line([100.0, 100.0, 100.0, 1.0], 0.09,
                 [x, 0.0, x, self.height],
                 c.transform, g);

            x += grid_height;

        }

    }

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

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

                // draw borders
                //rectangle([1.0, 1.0, 1.0, 0.3],
                //          [])
            }
        }

        if self.show_grid {
            self.draw_grid(&c, g);
        }

        text([0.5, 1.0, 0.0, 1.0], 15,
             &format!("iteration {}", self.engine.cur_iteration()),
             &mut self.resources.font,
             c.trans(10.0, 20.0).transform, g);

    }

}


fn main() {

    let mut game = Game::new(1024.0, 768.0);
    game.event_dispatcher();

}
