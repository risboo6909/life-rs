extern crate piston_window;
extern crate engine;

use piston_window::{Event, Button, Key, MouseButton};
use piston_window::graphics::{Context, line, rectangle};
use piston_window::{MouseCursorEvent, PressEvent, ReleaseEvent, UpdateEvent};

use super::{WindowBase, PostAction, WgpuGraphics};
use super::super::States;

use super::super::{CellProp, GraphicsWindow};
use super::Cam;

use engine::{Engine, CellDesc};

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
                // k = sqrt[(0.55*width*height) / (rows*cols*s^2*w*h)]

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

    fn paint(&mut self, c: Context, g: &mut WgpuGraphics) {

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
        if event.update_args().is_some() {
            if cur_state.get() == States::Working || cur_state.get() == States::StepByStep {
                if !self.render ||
                    Instant::now() - self.last_iter_time >= Duration::from_millis(2) ||
                    cur_state.get() == States::StepByStep {

                    self.engine.borrow_mut().iterations(1);
                    self.last_iter_time = Instant::now();

                    if cur_state.get() == States::StepByStep {
                        cur_state.set(States::Paused);
                    }
                }
            }
        }

        if let Some([x, y]) = event.mouse_cursor_args() {
            if cur_state.get() == States::Draw {
                self.born_or_kill(false, x, y);
            }
            self.last_pos = Some([x, y]);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            cur_state.set(States::Draw);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            if let Some(pos) = self.last_pos {
                self.born_or_kill(true, pos[0], pos[1]);
                cur_state.set(States::Paused);
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::P | Key::S => {
                    // always enable rendering in pause and step-by-step modes
                    self.render = true;
                }
                Key::G => self.show_grid = !self.show_grid,
                Key::Right => self.cam.move_right(),
                Key::Left => self.cam.move_left(),
                Key::Up => self.cam.move_up(),
                Key::Down => self.cam.move_down(),
                Key::NumPadMinus | Key::Minus => self.cam.zoom_out(),
                Key::NumPadPlus | Key::Equals => self.cam.zoom_in(),
                Key::R => {
                    // in pause mode 'R' is handled in UI layer, here it toggles rendering.
                    if cur_state.get() != States::Paused {
                        self.render = !self.render;
                    }
                }
                Key::F => self.fit_board(),
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Right | Key::Left | Key::Up | Key::Down => self.cam.reset_move_step(),
                _ => {}
            }
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

    fn draw_borders(&self, c: &Context, g: &mut WgpuGraphics) {

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

   fn draw_grid(&self, c: &Context, g: &mut WgpuGraphics) {

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
