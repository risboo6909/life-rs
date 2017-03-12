extern crate rand;
extern crate time;

use ::board::{Board, CellDesc};
use ::board::hashed::new as new_hashed;
use ::board::vect::new as new_vect;
use self::rand::distributions::{IndependentSample, Range};

#[derive(PartialEq, Copy, Clone)]
enum BoardType {
    Hashed,
    SymVec
}

pub struct Engine<'a> {
    board_type: BoardType,
    switch_inertia: usize,
    iters_from_prev_switch: usize,
    pub board: Board<'a>,
    pub iteration: usize,
    pub last_iter_time: f64,
}


impl<'a> Engine<'a> {

    pub fn new(cols: Option<usize>, rows: Option<usize>) -> Self {
        Engine {
            board_type: BoardType::Hashed,
            switch_inertia: 1,
            iters_from_prev_switch: 0,
            board: Self::new_board(BoardType::Hashed, cols, rows),
            iteration: 0,
            last_iter_time: 0f64
        }
    }

    fn new_board(board_type: BoardType, cols: Option<usize>, rows: Option<usize>) -> Board<'a> {
        let mut new_board;
        if board_type == BoardType::Hashed {
            new_board = Board::new(new_hashed(), cols, rows);
        }  else {
            new_board = Board::new(new_vect(), cols, rows);
        }
        new_board
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

        let mut board = Self::new_board(self.board_type,
                                        self.board.get_cols(), self.board.get_rows());

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

        let mut next_gen = Self::new_board(self.board_type,
                                           self.board.get_cols(), self.board.get_rows());

        let mut cells_checked = 0;

        let mut min_col = <isize>::max_value();
        let mut min_row = <isize>::max_value();
        let mut max_col = <isize>::min_value();
        let mut max_row = <isize>::min_value();

        for CellDesc { coord, gen, is_alive, .. } in self.board.into_iter() {
            let col = coord.col;
            let row = coord.row;

            if col < min_col {
                min_col = col;
            }

            if col > max_col {
                max_col = col;
            }

            if row < min_row {
                min_row = row;
            }

            if row > max_row {
                max_row = row;
            }

            cells_checked += 1;

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

        if self.iters_from_prev_switch > self.switch_inertia {

            if self.board_type == BoardType::Hashed {

                let density = (self.board.get_population() as f64) / (cells_checked as f64);

                if self.switch_inertia < 512 {
                    self.switch_inertia *= 2;
                }
                self.iters_from_prev_switch = 0;
                println!("switched to symvec board");
                self.switch_board();

            } else if self.board_type == BoardType::SymVec {

                let density = (self.board.get_population() as f64) / (cells_checked as f64);

                if density <= 0.04 {
                    if self.switch_inertia < 512 {
                        self.switch_inertia *= 2;
                    }
                    self.iters_from_prev_switch = 0;
                    println!("switched to hashed board");
                    self.switch_board();
                }

            }

        }

        self.iteration += 1;
        self.iters_from_prev_switch += 1;
    }

    pub fn switch_board(&mut self) {

        // switch internal board representation hash<->symvec

        let mut new_board;
        if self.board_type == BoardType::Hashed {
            new_board = Self::new_board(BoardType::SymVec, self.board.get_cols(), self.board.get_rows());
            self.board_type = BoardType::SymVec;
        }  else {
            new_board = Self::new_board(BoardType::Hashed, self.board.get_cols(), self.board.get_rows());
            self.board_type = BoardType::Hashed;
        }

        // copy old board content
        let cols = self.board.get_cols();
        let rows = self.board.get_rows();

        if cols.is_some() && rows.is_some() {
            for col in 0..cols.unwrap() {
                for row in 0..rows.unwrap() {
                    let gen = self.board.get_cell_gen(col as isize, row as isize);
                    if gen > 0 {
                        new_board.born_at_gen(col as isize, row as isize, gen);
                    }
                }
            }
        }

        self.set_board(new_board);
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
