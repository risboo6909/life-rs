extern crate rand;
extern crate time;

use ::board::{Board, CellDesc};
use ::board::hashed::new as new_hashed;
use ::board::vect::new as new_vect;
use self::rand::distributions::{IndependentSample, Range};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

const SWITCH_BOARD_INERTIA: usize = 128;
const ITERATIONS_TO_CLEANUP: usize = 1000;


#[derive(PartialEq, Copy, Clone)]
enum BoardType {
    Hashed,
    SymVec
}

pub struct Engine<'a> {
    board_type: BoardType,
    iters_from_prev_switch: usize,
    pub board: Board<'a>,
    pub iteration: usize,
    pub last_iter_time: f64,
}


struct MinMax {
    min: Option<isize>,
    max: Option<isize>
}

impl<'a> Engine<'a> {

    pub fn new(cols: Option<usize>, rows: Option<usize>) -> Self {
        let board_type = BoardType::Hashed;
        Engine {
            board_type: board_type,
            iters_from_prev_switch: SWITCH_BOARD_INERTIA,
            board: Self::new_board(board_type, cols, rows),
            iteration: 0,
            last_iter_time: 0f64
        }
    }

    fn new_board(board_type: BoardType, cols: Option<usize>, rows: Option<usize>) -> Board<'a> {
        if board_type == BoardType::Hashed {
            Board::new(new_hashed(), cols, rows)
        }  else {
            Board::new(new_vect(), cols, rows)
        }
    }

    fn clone_board(&self, board_type: BoardType) -> Board<'a> {

        let mut new_board = Self::new_board(board_type,
                                            self.board.get_cols(), self.board.get_rows());

        for CellDesc { coord, gen, is_alive, .. } in self.board.into_iter() {
            if is_alive {
                new_board.born_at_gen(coord.col, coord.row, gen);
            }
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

        let mut density_table: HashMap<isize, MinMax> = HashMap::new();

        for CellDesc { coord, gen, is_alive, .. } in self.board.into_iter() {

            let col = coord.col;
            let row = coord.row;

            if self.board_type == BoardType::Hashed {
                // for hashed board we maintain a hash table of
                // min and max coordinates of each row of the board
                match density_table.entry(row) {
                    Entry::Occupied(mut min_max_pair) => {
                        if col < min_max_pair.get().min.unwrap_or(isize::max_value()) {
                            let max = min_max_pair.get().max;
                            min_max_pair.insert(MinMax{min: Some(col), max: max});
                        } else if col > min_max_pair.get().max.unwrap_or(isize::min_value()) {
                            let min = min_max_pair.get().min;
                            min_max_pair.insert(MinMax{min: min, max: Some(col)});
                        }
                    },
                    Entry::Vacant(entry) => {
                        entry.insert(MinMax{min: None, max: None});
                    },
                }
            } else {
                cells_checked += 1;
            }

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

        // compute density of hashed board
        if self.board_type == BoardType::Hashed {
            for (_, v) in density_table.iter() {
                if let Some(x) = v.max {
                    cells_checked += x;
                }
                if let Some(x) = v.min {
                    cells_checked += x.abs();
                }
            }
        }

        let density = (self.board.get_population() as f64) / (cells_checked as f64);

        if density < 0.03 && self.board_type == BoardType::SymVec {
            if self.iters_from_prev_switch > SWITCH_BOARD_INERTIA {
                self.iters_from_prev_switch = 0;
                println!("switched to hashed board");
                self.switch_board();
            }
        } else if density >= 0.03 && self.board_type == BoardType::Hashed {
            if self.iters_from_prev_switch > SWITCH_BOARD_INERTIA {
                self.iters_from_prev_switch = 0;
                println!("switched to symvec board");
                self.switch_board();
            }
        }

        //println!("density {}", density);

        if (self.iteration % ITERATIONS_TO_CLEANUP) == 0 && self.board_type == BoardType::SymVec {
            // rebuild vector based board once per ITERATIONS_TO_CLEANUP iterations
            // to improve performance by removing empty cells
            let new_board = self.clone_board(self.board_type);
            self.set_board(new_board);
        }

        self.iteration += 1;
        self.iters_from_prev_switch += 1;
    }

    pub fn switch_board(&mut self) {

        // switch internal board representation hash<->symvec

        if self.board_type == BoardType::Hashed {
            self.board_type = BoardType::SymVec;
        }  else {
            self.board_type = BoardType::Hashed;
        }

        let new_board = self.clone_board(self.board_type);

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
