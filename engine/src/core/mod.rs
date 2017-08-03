extern crate rand;
extern crate time;

use board::{Board, CellDesc, HashedBoard, SymVecBoard, Coord};

use self::rand::distributions::{IndependentSample, Range};

use std::collections::HashMap;
use std::collections::hash_map::Entry;

const SWITCH_BOARD_INERTIA: usize = 128;
const ITERATIONS_TO_CLEANUP: usize = 1000;

pub mod loader;


#[derive(PartialEq, Copy, Clone)]
enum BoardType {
    Hashed,
    SymVec
}

pub struct Engine {
    cols: Option<usize>,
    rows: Option<usize>,

    board_type: BoardType,
    iters_from_prev_switch: usize,
    pub board: Board,
    pub iteration: usize,
    pub last_iter_time: f64,
}

struct MinMax {
    min: Option<isize>,
    max: Option<isize>
}


impl Engine {

    pub fn new(config_vec: Option<Vec<Coord>>, cols: Option<usize>, rows: Option<usize>) -> Self {
        let board_type = BoardType::Hashed;

        let mut engine = Engine {
            cols: cols,
            rows: rows,

            board_type: board_type,
            iters_from_prev_switch: SWITCH_BOARD_INERTIA,
            board: Self::new_board(board_type, cols, rows),
            iteration: 0,
            last_iter_time: 0f64
        };

        if let Some(board_config) = config_vec {
            engine.board.set_predefined(board_config);
        };

        engine
    }

    pub fn set_predefined(&mut self, board_config: Vec<Coord>) {
        self.board.set_predefined(board_config);
    }

    pub fn reset(&mut self) {
        self.board = Self::new_board(self.board_type, self.cols, self.rows);
        self.iteration = 0;
        self.last_iter_time = 0f64;
    }

    fn new_board(board_type: BoardType, cols: Option<usize>, rows: Option<usize>) -> Board {
        if board_type == BoardType::Hashed {
            Board::new(HashedBoard::new(), cols, rows)
        }  else {
            Board::new(SymVecBoard::new(), cols, rows)
        }
    }

    fn clone_board(&self, board_type: BoardType) -> Board {

        let mut new_board = Self::new_board(board_type,
                                            self.board.get_cols(), self.board.get_rows());

        for CellDesc { coord, gen, is_alive, .. } in self.board.iter() {
            if is_alive {
                new_board.born_at_gen(coord.col, coord.row, gen);
            }
        }

        new_board

    }

    pub fn cur_iteration(&self) -> usize {
        self.iteration
    }

    pub fn get_last_iter_time(&self) -> f64 {
        self.last_iter_time
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn create_random(&self, p: f64) -> Board {

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

        for CellDesc { coord, gen, is_alive, .. } in self.board.iter() {

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
