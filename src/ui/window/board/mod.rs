extern crate piston_window;


use piston_window::{Context, line, rectangle, Transformed};
use super::{ActiveWindow, WindowBase};

use super::super::super::engine::Engine;
pub use super::super::super::board::{Board, CellDesc};
use super::super::super::structs::{CellProp, GraphicsWindow};

use opengl_graphics::GlGraphics;

use std::rc::Rc;
use std::cell::RefCell;

use cam::Cam;

pub struct GameBoard<'a> {

    window: Rc<GraphicsWindow>,
    engine: Rc<RefCell<Engine<'a>>>,

    cell: CellProp,
    cam: Cam,

    show_grid: bool,
    render: bool,

}

trait GameBoardTrait: ActiveWindow {

    fn get_right_border(&self) -> f64;
    fn get_left_border(&self) -> f64;
    fn get_top_border(&self) -> f64;
    fn get_bottom_border(&self) -> f64;

    fn to_logical(&self, x: f64, y: f64) -> (isize, isize);
    fn to_screen(&self, col: isize, row: isize) -> (f64, f64);
    fn born_or_kill(&mut self, kill_alive: bool, x: f64, y: f64);

    fn get_color(gen: usize) -> [f32; 4];

    fn draw_borders(&self, c: &Context, g: &mut GlGraphics);
    fn draw_grid(&self, c: &Context, g: &mut GlGraphics);

}

impl<'a> WindowBase for GameBoard<'a> {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        if self.render {
            {
                let engine = self.engine.borrow();

                for CellDesc { coord, gen, is_alive, .. } in engine.get_board().into_iter() {
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

}

impl<'a> ActiveWindow for GameBoard<'a> {

    fn event_dispatcher(&self) {

    }

}

impl<'a> GameBoardTrait for GameBoard<'a> {

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

pub fn new<'a>(window: Rc<GraphicsWindow>, engine: Rc<RefCell<Engine<'a>>>) -> GameBoard<'a> {

    GameBoard {
        window: window,
        engine: engine,

        cell: CellProp::new(10.0, 10.0),
        cam: Cam::new(0.0, 0.0),

        show_grid: true,
        render: true,
    }

}
