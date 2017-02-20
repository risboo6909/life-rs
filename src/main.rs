/// The Gmae of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;

mod board;
mod engine;
mod cam;
mod structs;
mod symvec;

use cam::Cam;
use structs::{CellProp, GameWindow};

use find_folder::Search;
use piston_window::{OpenGL, Context, text, clear, rectangle, line,
                    Transformed, Event, Button, Input,
                    MouseButton, Key, PistonWindow, WindowSettings, Motion};

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use engine::Engine;
use board::{Board, CellDesc};
use std::time::{Instant, Duration};

const OPENGL: piston_window::OpenGL = OpenGL::V3_2;

const GREEN: [f32; 4] = [0.5, 1.0, 0.0, 1.0];
const GRAY: [f32; 4] = [100.0, 100.0, 100.0, 1.0];
const RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];


#[derive(PartialEq)]
enum State {
    Working,
    Draw,
    Paused,
    Help,
}


struct Resources {
    font: GlyphCache<'static>
}


struct Game {

    window: GameWindow,
    cell: CellProp,

    show_grid: bool,
    render: bool,

    engine: Engine,
    cam: Cam,
    cur_state: State,

    resources: Resources,
}


impl Game {
    fn new(width: f64, height: f64) -> Game {
        let window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width as u32, height as u32]
        ).opengl(OPENGL)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let game_board = Board::new(Some(200), Some(200));

        Game {

            window: GameWindow::new(width, height, window),

            cell: CellProp::new(10.0, 10.0),

            // show grid
            show_grid: true,

            // enable/disable rendering
            render: true,

            engine: Engine::new(game_board),

            cam: Cam::new(0.0, 0.0),

            // current game state
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

            let event = { self.window.get_window().borrow_mut().next() };

            match event {
                Some(e) => {
                    match e {
                        Event::Render(args) => {
                            gl.draw(args.viewport(), |c, g| self.paint(c, g));
                        }

                        Event::Update(_) => {
                            if self.cur_state == State::Working {
                                if !self.render || Instant::now() - last_iter_time >= Duration::from_millis(3) {
                                    self.engine.one_iteration();
                                    last_iter_time = Instant::now();
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
                                let board = self.engine.create_random(0.2);
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
        // converts from logical board coordinates into screen coordinates
        // taking in account current camera position and scale

        let x = col as f64 * self.cell.get_width(&self.cam) + self.window.get_half_width() -
            self.cell.get_half_width(&self.cam);

        let y = row as f64 * self.cell.get_height(&self.cam) + self.window.get_half_height() -
            self.cell.get_half_height(&self.cam);

        self.cam.translate(x, y)
    }

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize) {
        let (x, y) = self.cam.translate_inv(x, y);

        let mut offset_x = x - self.window.get_half_width();
        let mut offset_y = y - self.window.get_half_height();

        if offset_x < 0.0 {
            offset_x -= self.cell.get_half_width(&self.cam);
        } else if offset_x > 0.0 {
            offset_x += self.cell.get_half_width(&self.cam);
        }

        if offset_y < 0.0 {
            offset_y -= self.cell.get_half_height(&self.cam);
        } else if offset_y > 0.0 {
            offset_y += self.cell.get_half_height(&self.cam);
        }

        let col = (offset_x / self.cell.get_width(&self.cam)) as isize;
        let row = (offset_y / self.cell.get_height(&self.cam)) as isize;

        (col, row)
    }

    #[inline]
    fn get_right_border(&self, cols: usize) -> f64 {
        self.cam.translate_x(self.window.get_half_width() +
            cols as f64 * self.cell.get_half_width(&self.cam))
    }

    #[inline]
    fn get_left_border(&self, cols: usize) -> f64 {
        self.cam.translate_x(self.window.get_half_width() -
            cols as f64 * self.cell.get_half_width(&self.cam) - 1.0)
    }

    #[inline]
    fn get_top_border(&self, rows: usize) -> f64 {
        self.cam.translate_y(self.window.get_half_height() -
            rows as f64 * self.cell.get_half_height(&self.cam) - 1.0)
    }

    #[inline]
    fn get_bottom_border(&self, rows: usize) -> f64 {
        self.cam.translate_y(self.window.get_half_height() +
            rows as f64 * self.cell.get_half_height(&self.cam))
    }

    fn draw_grid(&self, c: &Context, g: &mut GlGraphics) {

        // TODO: Refactor

        let grid_width = self.cell.get_width(&self.cam);
        let grid_height = self.cell.get_height(&self.cam);

        let offset_x = 0.5 * grid_width - 0.5 * self.window.get_width() % grid_width;
        let offset_y = 0.5 * grid_height - 0.5 * self.window.get_height() % grid_height;

        let mut x = self.cam.get_x() - offset_x;
        let mut y = self.cam.get_y() - offset_y;

        let mut right_offset_x = self.window.get_width();
        let mut left_offset_x = 0.0;

        let mut bottom_offset_y = self.window.get_height();
        let mut top_offset_y = 0.0;

        if let Some(cols) = self.engine.get_board().get_cols() {
            right_offset_x = self.get_right_border(cols);
            left_offset_x = self.get_left_border(cols);
        }

        if let Some(rows) = self.engine.get_board().get_rows() {
            top_offset_y = self.get_top_border(rows);
            bottom_offset_y = self.get_bottom_border(rows);
        }

        // horizontal lines
        while y < self.window.get_height() as f64 {
            if y > top_offset_y && y < bottom_offset_y {
                line(GRAY, 0.09,
                     [left_offset_x, y, right_offset_x, y],
                     c.transform, g);
            }

            y += grid_height;
        }

        // vertical lines
        while x < self.window.get_width() as f64 {
            if x > left_offset_x && x < right_offset_x {
                line(GRAY, 0.09,
                     [x, top_offset_y, x, bottom_offset_y],
                     c.transform, g);
            }

            x += grid_height;
        }
    }

    fn draw_borders(&self, c: &Context, g: &mut GlGraphics) {
        // draw borders

        let mut right_offset_x = self.window.get_width();
        let mut left_offset_x = 0.0;

        let mut bottom_offset_y = self.window.get_height();
        let mut top_offset_y = 0.0;

        if let Some(cols) = self.engine.get_board().get_cols() {
            right_offset_x = self.get_right_border(cols);
            left_offset_x = self.get_left_border(cols);
        }

        if let Some(rows) = self.engine.get_board().get_rows() {
            top_offset_y = self.get_top_border(rows);
            bottom_offset_y = self.get_bottom_border(rows);
        }

        if let Some(_) = self.engine.get_board().get_cols() {
            // draw right border

            line(RED, 0.3,
                 [right_offset_x, top_offset_y, right_offset_x, bottom_offset_y],
                 c.transform, g);

            // draw left border

            line(RED, 0.3,
                 [left_offset_x, top_offset_y, left_offset_x, bottom_offset_y],
                 c.transform, g);
        }

        if let Some(_) = self.engine.get_board().get_rows() {
            // draw top border

            line(RED, 0.3,
                 [left_offset_x, top_offset_y, right_offset_x, top_offset_y],
                 c.transform, g);

            // draw bottom border

            line(RED, 0.3,
                 [left_offset_x, bottom_offset_y, right_offset_x, bottom_offset_y],
                 c.transform, g);
        }
    }

    fn draw_hud(&mut self, c: &Context, g: &mut GlGraphics) {
        text(GREEN, 15,
             &format!("iteration {}", self.engine.cur_iteration()),
             &mut self.resources.font,
             c.trans(10.0, 20.0).transform, g);

        text(GREEN, 15,
             &format!("population {}", self.engine.get_board().get_population()),
             &mut self.resources.font,
             c.trans(150.0, 20.0).transform, g);
    }

    fn get_color(gen: usize) -> [f32; 4] {
        let gen = gen as f32;

        let mut r = 255.0;

        if gen < 255.0 {
            r = gen / 255.0;
        }

        let tmp = [r, 1.0, 0.0, 0.5];

        tmp
    }

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        clear([0.0, 0.0, 0.0, 1.0], g);

        if self.render {
            {
                let board = self.engine.get_board();

                for CellDesc { coord, gen, is_alive, .. } in board.into_iter() {
                    if is_alive {
                        let (x, y) = self.to_screen(coord.col, coord.row);
                        rectangle(Game::get_color(gen), [x, y,
                            self.cell.get_width(&self.cam),
                            self.cell.get_height(&self.cam)],
                                  c.transform, g);
                    }
                }
            }

            if self.show_grid {
                self.draw_grid(&c, g);
            }

            self.draw_borders(&c, g);
        }

        // hud is always visible
        self.draw_hud(&c, g);
    }
}


fn main() {
    let mut game = Game::new(1024.0, 768.0);
    game.event_dispatcher();
}
