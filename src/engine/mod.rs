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
    pub board: Board<'a>,
    pub iteration: usize,
    pub last_iter_time: f64,
}


impl<'a> Engine<'a> {

    pub fn new(cols: Option<usize>, rows: Option<usize>) -> Self {
        Engine {
            board_type: BoardType::Hashed,
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

        if self.board_type == BoardType::Hashed && self.board.get_population() > 1500 {
            self.switch_board();
        } else if self.board_type == BoardType::SymVec && self.board.get_population() <= 1500 {
            self.switch_board();
        }

        self.iteration += 1;
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
