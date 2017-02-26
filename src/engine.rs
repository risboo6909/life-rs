extern crate rand;
extern crate time;

use ::board::{Board, BoardInternal, HashBased, SymVecBased};
use ::board::CellDesc;
use self::rand::distributions::{IndependentSample, Range};


pub struct Engine<'a> {
    pub board: Board<'a>,
    pub iteration: usize,
    pub last_iter_time: f64,
}


impl<'a> Engine<'a> {

    pub fn new(cols: Option<usize>, rows: Option<usize>) -> Self {
        Engine {
            board: Board::new(cols, rows),
            iteration: 0,
            last_iter_time: 0f64
        }
    }

    pub fn from_file() {}

    pub fn cur_iteration(&self) -> usize {
        self.iteration
    }

    pub fn get_last_iter_time(&self) -> f64 {
        self.last_iter_time
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn set_board(&mut self, board: Board<'a>) {
        self.board = board;
    }

    pub fn get_board_mut(&mut self) -> &mut Board<'a> {
        &mut self.board
    }

    pub fn create_random(&self, p: f64) -> Board<'a> {

        let mut board = Board::new(self.board.get_cols(), self.board.get_rows());

        let cols = self.board.get_cols();
        let rows = self.board.get_rows();

        let between = Range::new(0f64, 1.);
        let mut rng = rand::thread_rng();

        if cols.is_some() && rows.is_some() {
            for col in 0..cols.unwrap() {
                for row in 0..rows.unwrap() {
                    let rval = between.ind_sample(&mut rng);
                    if rval <= p {
                        board.born_at(col as isize, row as isize);
                    }
                }
            }
        }

        board
    }

    pub fn one_iteration(&mut self) {
        let mut next_gen = Board::new(self.board.get_cols(), self.board.get_rows());

        for CellDesc { coord, gen, is_alive, .. } in self.board.into_iter() {
            let col = coord.col;
            let row = coord.row;

            // check game rules against current cell
            let neighbours = self.board.get_vicinity(col, row);

            if is_alive {
                let neighbours_cnt = neighbours.into_iter().filter(|&x| x).count();
                // any live cell with fewer than two live neighbours dies,
                // as if caused by underpopulation.

                // any live cell with more than three live neighbours
                // dies, as if by overpopulation.

                // any live cell with two or three live neighbours
                // lives on to the next generation.
                if neighbours_cnt == 2 || neighbours_cnt == 3 {
                    next_gen.born_at_gen(col, row, gen + 1);
                }
            } else {
                // any dead cell with exactly three live neighbours becomes
                // a live cell, as if by reproduction.
                if neighbours.into_iter().filter(|&x| x).count() == 3 {
                    next_gen.born_at(col, row);
                }
            }
        }

        self.board = next_gen;
        self.iteration += 1;
    }

    pub fn iterations(&mut self, n: u64) -> f64 {
        let st = time::precise_time_s();
        for _ in 0..n {
            self.one_iteration();
        }
        self.last_iter_time = time::precise_time_s() - st;
        self.last_iter_time
    }
}
