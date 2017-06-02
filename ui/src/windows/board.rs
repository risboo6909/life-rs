extern crate piston_window;
extern crate engine;

use piston_window::{Context, Event, Input, Button, Key,
                    MouseButton, Motion, line, rectangle};

use super::{WindowBase, PostAction};
use super::super::States;

use super::super::{CellProp, GraphicsWindow};
use super::Cam;

use engine::{Engine, CellDesc};

use opengl_graphics::GlGraphics;

use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::time::{Instant, Duration};


pub struct GameBoard {

    window: Rc<GraphicsWindow>,
    engine: Rc<RefCell<Engine>>,

    cell: CellProp,
    cam: Cam,

    show_grid: bool,
    render: bool,

    last_iter_time: Instant,
    last_pos: Option<[f64; 2]>,

}

impl GameBoard {

    pub fn new(window: Rc<GraphicsWindow>, engine: Rc<RefCell<Engine>>) -> GameBoard {

        GameBoard {

            window: window,
            engine: engine,

            cell: CellProp::new(10.0, 10.0),
            cam: Cam::new(0.0, 0.0),

            show_grid: true,
            render: true,

            last_iter_time: Instant::now(),
            last_pos: None,

        }

    }

    fn fit_board(&mut self) {

        // fit game board on a screen

        if let Some(board_cols) = self.engine.borrow().get_board().get_cols() {
            if let Some(board_rows) = self.engine.borrow().get_board().get_rows() {

                // try to make all borders to be visible if possible

                // grid must take 55% of the window
                let target_area = 0.55 * self.window.get_width() * self.window.get_height();

                // the derivation is as follows:
                // rows*cols*s*k*w*s*k*h = 0.55*width*height
                // k^2 = (0.55*width*height) / (rows*cols*s^2*w*h)

                let cell_area = self.cell.get_width(&self.cam) * self.cell.get_height(&self.cam);
                let current_area = cell_area * (board_cols * board_rows) as f64;

                let scale_factor = (target_area / current_area).sqrt();
                let scale_coeff = self.cam.get_scale();

                self.cam.set_scale((scale_factor * scale_coeff));

            }
        }

        // position camera to look at the center of a board

        let (center_x, center_y) = self.get_board_center();

        let delta_x = match(center_x) {
            Some(x) => x - 0.5 * self.window.get_width(),
            None => 0.0
        };

        let delta_y = match(center_y) {
            Some(y) => y - 0.5 * self.window.get_height(),
            None => 0.0
        };

        self.cam.set_pos_delta(-delta_x, -delta_y);

    }

}

impl WindowBase for GameBoard {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        if self.render {
            {
                let engine = self.engine.borrow();

                for CellDesc { coord, gen, is_alive, .. } in engine.get_board().iter() {
                    if is_alive {
                        let (x, y) = self.to_screen(coord.col, coord.row);
                        rectangle(GameBoard::get_color(gen), [x, y,
                            self.cell.get_width(&self.cam),
                            self.cell.get_height(&self.cam)],
                                  c.transform, g);
                    }
                }
            }
        }

        if self.show_grid {
            self.draw_grid(&c, g);
        }

        self.draw_borders(&c, g);
    }

    fn event_dispatcher(&mut self, event: &Event, cur_state: &Cell<States>) -> PostAction {

        match event {

            &Event::Update(_) => {

                if cur_state.get() == States::Working || cur_state.get() == States::StepByStep {
                    if !self.render ||
                        Instant::now() - self.last_iter_time >= Duration::from_millis(3) ||
                        cur_state.get() == States::StepByStep {

                        self.engine.borrow_mut().iterations(1);
                        self.last_iter_time = Instant::now();

                        if cur_state.get() == States::StepByStep {
                            cur_state.set(States::Paused);
                        }

                    }
                }

            }

            &Event::Input(Input::Press(Button::Keyboard(Key::P))) => {
                // always enable rendering in pause mode
                self.render = true;
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::S))) => {
                // always enable rendering in step by step mode
                self.render = true;
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::G))) => {
                // show/hide grid
                self.show_grid = !self.show_grid;
            }

            // mouse controls ->
            &Event::Input(Input::Press(Button::Mouse(MouseButton::Left))) => {
                cur_state.set(States::Draw);
            }

            &Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
                if self.last_pos.is_some() {
                    let pos = self.last_pos.unwrap();
                    self.born_or_kill(true, pos[0], pos[1]);

                    cur_state.set(States::Paused);
                }
            }

            &Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
                if cur_state.get() == States::Draw {
                    self.born_or_kill(false, x, y);
                }
                self.last_pos = Some([x, y]);
            }
            // mouse control <-

            // movements control ->
            &Event::Input(Input::Press(Button::Keyboard(Key::Right))) => {
                self.cam.move_right();
            }

            &Event::Input(Input::Release(Button::Keyboard(Key::Right))) => {
                self.cam.reset_move_step();
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::Left))) => {
                self.cam.move_left();
            }

            &Event::Input(Input::Release(Button::Keyboard(Key::Left))) => {
                self.cam.reset_move_step();
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::Up))) => {
                self.cam.move_up();
            }

            &Event::Input(Input::Release(Button::Keyboard(Key::Up))) => {
                self.cam.reset_move_step();
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::Down))) => {
                self.cam.move_down();;
            }

            &Event::Input(Input::Release(Button::Keyboard(Key::Down))) => {
                self.cam.reset_move_step();
            }
            // movements control <-

            // zoom out ->
            &Event::Input(Input::Press(Button::Keyboard(Key::NumPadMinus))) => {
                self.cam.zoom_out();
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::Minus))) => {
                self.cam.zoom_out();
            }
            // zoom out <-

            // zoom in ->
            &Event::Input(Input::Press(Button::Keyboard(Key::NumPadPlus))) => {
                self.cam.zoom_in();
            }

            // use "Equals" instead of "Plus" to avoid holding shift key requirement
            &Event::Input(Input::Press(Button::Keyboard(Key::Equals))) => {
                self.cam.zoom_in();
            }
            // zoom in <-

            // misc controls ->
            &Event::Input(Input::Press(Button::Keyboard(Key::R))) => {
                // in pause mode - fill board with a random pattern
                if cur_state.get() != States::Paused {
                    // otherwise enable/disable rendering
                    self.render = !self.render;
                }
            }

            &Event::Input(Input::Press(Button::Keyboard(Key::F))) => {
                // reset camera coordinates to defaults
                //self.cam.reset();
                self.fit_board();
            }
            // misc controls <-

            _ => {}

        }

        PostAction::Transfer

    }

}

impl GameBoard {

    #[inline]
    fn get_right_border(&self) -> f64 {
        // get absolute screen coordinate of right border of a board
        if let Some(cols) = self.engine.borrow().get_board().get_cols() {
            let x = self.cam.translate_x(self.window.get_half_width() +
                0.5 * cols as f64 * self.cell.get_width(&self.cam));
            if cols % 2 == 0 { x - self.cell.get_half_height(&self.cam) } else { x }
        } else { self.window.get_width() }
    }

    #[inline]
    fn get_left_border(&self) -> f64 {
        // get absolute screen coordinate of left border of a board
        let cols = match self.engine.borrow().get_board().get_cols() {
            Some(cols) => cols,
            None => (self.window.get_width() / self.cell.get_width(&self.cam)) as usize
        };
        let x = self.cam.translate_x(self.window.get_half_width() -
            0.5 * cols as f64 * self.cell.get_width(&self.cam));
        if cols % 2 == 0 { x - self.cell.get_half_height(&self.cam) } else { x }
    }

    #[inline]
    fn get_top_border(&self) -> f64 {
        // get absolute screen coordinate of top border of a board
        let rows = match self.engine.borrow().get_board().get_rows() {
            Some(rows) => rows,
            None => (self.window.get_height() / self.cell.get_height(&self.cam)) as usize
        };
        let y = self.cam.translate_y(self.window.get_half_height() -
            0.5 * rows as f64 * self.cell.get_height(&self.cam));
        if rows % 2 == 0 { y - self.cell.get_half_height(&self.cam) } else { y }
    }

    #[inline]
    fn get_bottom_border(&self) -> f64 {
        // get absolute screen coordinate of bottom border of a board
        if let Some(rows) = self.engine.borrow().get_board().get_rows() {
            let y = self.cam.translate_y(self.window.get_half_height() +
                0.5 * rows as f64 * self.cell.get_height(&self.cam));
            if rows % 2 == 0 { y - self.cell.get_half_height(&self.cam) } else { y }
        } else { self.window.get_height() }
    }

    #[inline]
    fn get_board_center(&self) -> (Option<f64>, Option<f64>) {

        let center_x = match(self.engine.borrow().get_board().get_cols()) {
            Some(_) => Some(0.5 * (self.get_left_border() + self.get_right_border())),
            None => None
        };
        let center_y = match(self.engine.borrow().get_board().get_cols()) {
            Some(_) => Some(0.5 * (self.get_bottom_border() + self.get_top_border())),
            None => None
        };

        (center_x, center_y)
    }

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize) {
        let (x, y) = self.cam.translate_inv(x, y);

        let mut offset_x = x - self.window.get_half_width();
        let mut offset_y = y - self.window.get_half_height();

        // TODO: Ensure this needed

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

    fn to_screen(&self, col: isize, row: isize) -> (f64, f64) {
        // converts from logical board coordinates into screen coordinates
        // taking into account current camera position and scale

        // suppose that screen center goes through the center of a cell
        // with coordinates (0, 0)
        //
        //               ^
        //               |
        //               |
        //              [|] - - - >

        let x = col as f64 * self.cell.get_width(&self.cam) + self.window.get_half_width() -
            self.cell.get_half_width(&self.cam);

        let y = row as f64 * self.cell.get_height(&self.cam) + self.window.get_half_height() -
            self.cell.get_half_height(&self.cam);

        self.cam.translate(x, y)
    }

    fn born_or_kill(&mut self, kill_alive: bool, x: f64, y: f64) {
        let (col, row) = self.to_logical(x, y);
        let mut engine = self.engine.borrow_mut();

        let board = engine.get_board_mut();

        if kill_alive && board.is_alive(col, row) {
            board.kill_at(col, row);
        } else {
            board.born_at(col, row);
        }
    }

    fn get_color(gen: usize) -> [f32; 4] {
        let r = 1.0_f64.min(50.0*gen as f64/256.0);
        [r as f32, 1.0 - r as f32, 0.0, 0.5]
    }

   fn draw_borders(&self, c: &Context, g: &mut GlGraphics) {

        // draw borders
        let right_offset_x = self.get_right_border();
        let left_offset_x = self.get_left_border();

        let top_offset_y = self.get_top_border();
        let bottom_offset_y = self.get_bottom_border();

        if let Some(_) = self.engine.borrow_mut().get_board().get_cols() {
            // draw right border

            line(super::RED, 0.3,
                 [right_offset_x, top_offset_y, right_offset_x, bottom_offset_y],
                 c.transform, g);

            // draw left border

            line(super::RED, 0.3,
                 [left_offset_x, top_offset_y, left_offset_x, bottom_offset_y],
                 c.transform, g);
        }

        if let Some(_) = self.engine.borrow_mut().get_board().get_rows() {
            // draw top border

            line(super::RED, 0.3,
                 [left_offset_x, top_offset_y, right_offset_x, top_offset_y],
                 c.transform, g);

            // draw bottom border

            line(super::RED, 0.3,
                 [left_offset_x, bottom_offset_y, right_offset_x, bottom_offset_y],
                 c.transform, g);
        }
   }

   fn draw_grid(&self, c: &Context, g: &mut GlGraphics) {

       let right_offset_x = self.get_right_border();
       let left_offset_x = self.get_left_border();

       let top_offset_y = self.get_top_border();
       let bottom_offset_y = self.get_bottom_border();

       let mut y = top_offset_y;

       // horizontal lines
       while y < bottom_offset_y {

           line(super::GRAY, 0.09,
                [left_offset_x, y, right_offset_x, y],
                c.transform, g);

           y += self.cell.get_height(&self.cam);
       }

       let mut x = left_offset_x;

       // vertical lines
       while x < right_offset_x {

           line(super::GRAY, 0.09,
                [x, top_offset_y, x, bottom_offset_y],
                c.transform, g);

           x += self.cell.get_width(&self.cam);
       }
   }

}
