/// Expandable game board, it expands on demand in all four directions, constraints
/// can be defined to limit maxmimum memory consumption (thus the game board
/// will behave like a torus), otherwise it will expand "forever" consuming memory :]
///
/// ```
/// let mut my_board = Board::new(Some(30), Some(30));
/// my_board.born_at(20, 20);
/// ```
/// Example above will create a board with maximum height and width
/// equal to 30, so it will contain 900 cells.
///
/// In fact, board of any size can be created and it will expand on demand,
/// therefor it is recommended to initially create smallest board which
/// will contain initial configuration.

use std::cmp::max;
use std::collections::HashMap;
use std::collections::hash_map::{IntoIter, Iter};


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub col: isize,
    pub row: isize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Cell {
    Empty,
    // occupied cell contains its generation
    Occupied { gen: usize }
}

pub struct CellDesc {
    pub coord: Coord,
    pub gen: usize,
    pub is_alive: bool,
    pub new_line: bool,
}

pub struct Board {
    cells: HashMap<(isize, isize), Cell>,

    rows: Option<usize>,
    cols: Option<usize>,

    half_cols: Option<isize>,
    half_rows: Option<isize>,
}

impl Board {

    pub fn new(width: Option<usize>, height: Option<usize>) -> Board {

        Board { cells: HashMap::new(),
                cols: width,
                rows: height,
                half_cols: width.map(|x| (x / 2) as isize),
                half_rows: height.map(|x| (x / 2) as isize),
              }
    }

    #[inline]
    fn cycle(x: isize, min_val: isize, max_val: isize) -> isize {

        // TODO: add description

        let cnt = max_val - min_val;

        assert!(cnt > 0);

        if x < min_val {
            max_val - (min_val - x) % (cnt + 1)
        } else if x > max_val - 1 {
            min_val + (x - max_val) % cnt
        } else {
            x
        }

    }

    #[inline]
    fn bound_coordinate(size: Option<isize>, coord: isize) -> isize {

        match size {

            Some(x) => {
                    if coord >= x || coord < -x {
                        Board::cycle(coord, -x, x)
                    } else { coord }
                },

            None => coord

        }

    }

    #[inline]
    fn constrain_board(&self, col: isize, row: isize) -> (isize, isize) {

        // ensure cell coordinates lie inside limits

        let col = Board::bound_coordinate(self.half_cols, col);
        let row = Board::bound_coordinate(self.half_rows, row);

        (col, row)
    }

    pub fn ensure_cell(&mut self, col: isize, row: isize) {
        let coords = self.constrain_board(col, row);
        if self.cells.get(&coords) == None {
            self.cells.insert(coords, Cell::Empty);
        }
    }

    pub fn born_at_gen(&mut self, col: isize, row: isize, gen: usize) {

        self.ensure_cell(col, row);

        // we must allocate 8 cells around current cell because
        // new species can potentially be borned there, so we
        // have to check them on next iteration

        self.ensure_cell(col - 1, row);
        self.ensure_cell(col - 1, row - 1);
        self.ensure_cell(col, row - 1);
        self.ensure_cell(col + 1, row - 1);
        self.ensure_cell(col + 1, row);
        self.ensure_cell(col + 1, row + 1);
        self.ensure_cell(col, row + 1);
        self.ensure_cell(col - 1, row + 1);

        let coords = self.constrain_board(col, row);
        self.cells.insert(coords, Cell::Occupied { gen: gen });

    }

    pub fn born_at(&mut self, col: isize, row: isize) {
        self.born_at_gen(col, row, 1);
    }

    #[inline]
    pub fn kill_at(&mut self, col: isize, row: isize) {
        let coords = self.constrain_board(col, row);    
        self.cells.remove(&coords);
    }

    #[inline]
    pub fn is_alive(&self, col: isize, row: isize) -> bool {
        self.get_cell(col, row) != Cell::Empty
    }

    pub fn get_cell(&self, col: isize, row: isize) -> Cell {
        // if cell is not yet initialized it is considered as free
        match self.cells.get(&self.constrain_board(col, row)) {
            Some(x) => {
                if let &Cell::Occupied {gen: gen} = x {
                    return *x
                } else {
                    return Cell::Empty
                }
            }
            None => Cell::Empty
        }
    }

    pub fn get_cell_gen(&self, col: isize, row: isize) -> usize {
        match self.get_cell(col ,row) {
            Cell::Occupied{gen} => gen,
            Cell::Empty => 0
        }
    }

    pub fn get_vicinity(&self, col: isize, row: isize) -> Vec<bool> {

        // get contents of 8 neighbours of a given cell

        let neighbours = vec![
            self.is_alive(col - 1, row),
            self.is_alive(col - 1, row - 1),
            self.is_alive(col, row - 1),
            self.is_alive(col + 1, row - 1),
            self.is_alive(col + 1, row),
            self.is_alive(col + 1, row + 1),
            self.is_alive(col, row + 1),
            self.is_alive(col - 1, row + 1),
        ];

        neighbours
    }

    #[inline]
    pub fn get_cols(&self) -> Option<usize> {
        self.cols
    }

    #[inline]
    pub fn get_rows(&self) -> Option<usize> {
        self.rows
    }

}

impl<'a> IntoIterator for &'a Board {

    type Item = CellDesc;
    type IntoIter = BoardIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIntoIterator { board: &self,
                            cell_iter: Box::new(self.cells.iter()) }
    }

}

pub struct BoardIntoIterator<'a> {
    board: &'a Board,
    cell_iter: Box<Iter<'a, (isize, isize), Cell>>,
}

impl<'a> Iterator for BoardIntoIterator<'a> {

    type Item = CellDesc;

    fn next(&mut self) -> Option<CellDesc> {

        match self.cell_iter.next() {

            Some(e) => {

                let &(col, row) = e.0;
                let &cell = e.1;

                let gen = match cell {
                    Cell::Occupied{gen} => gen,
                    Cell::Empty => 0
                };

                Some(CellDesc { coord: Coord { col: col, row: row },
                                gen: gen,
                                is_alive: self.board.is_alive(col, row),
                                new_line: false })

            }

            None => None
        }

    }
}


#[test]
fn test_board_ok() {

    let mut my_board = Board::new(Some(10), Some(10));

    // set some existing cells
    my_board.born_at(0, 0);
    my_board.born_at(4, 4);

    // extend board by one cell
    my_board.born_at(5, 2);

    // test allocated cells
    assert_eq!(my_board.get_cell(0, 0), Cell::Occupied { gen: 1 });
    assert_eq!(my_board.get_cell(4, 4), Cell::Occupied { gen: 1 });

    // test previously expanded cell
    assert_eq!(my_board.get_cell(5, 2), Cell::Occupied { gen: 1 });

    // test existing cell
    assert_eq!(my_board.get_cell(2, 2), Cell::Empty);

    // check extended cell
    assert_eq!(my_board.get_cell(5, 3), Cell::Empty);

    my_board.kill_at(0, 0);
    assert_eq!(my_board.get_cell(0, 0), Cell::Empty);

}

#[test]
fn test_board_iter() {

    let mut my_board = Board::new(Some(10), Some(10));

    my_board.born_at(0, 0);
    my_board.born_at(1, 1);
    my_board.born_at(2, 2);
    my_board.born_at(3, 3);
    my_board.born_at(4, 4);

    let mut ctr = 0;

    for CellDesc { coord, is_alive, .. } in my_board.into_iter() {
        if is_alive {
            ctr += 1;
        }
    }

    assert!(ctr == 5);
}

#[test]
fn test_glyder() {
    let mut my_board = Board::new(Some(10), Some(10));

    my_board.born_at(0, 0);
    my_board.born_at(1, 1);
    my_board.born_at(1, 2);
    my_board.born_at(0, 2);
    my_board.born_at(-1, 2);

    assert_eq!(my_board.is_alive(0, 0), true);
    assert_eq!(my_board.is_alive(1, 1), true);
    assert_eq!(my_board.is_alive(1, 2), true);
    assert_eq!(my_board.is_alive(0, 2), true);
    assert_eq!(my_board.is_alive(-1, 2), true);
}

#[test]
fn test_cycle() {

    assert_eq!(Board::cycle(0, -5, 5), 0);
    assert_eq!(Board::cycle(-5, -5, 5), -5);
    assert_eq!(Board::cycle(5, -5, 5), -5);
    assert_eq!(Board::cycle(6, -5, 5), -4);
    assert_eq!(Board::cycle(-6, -5, 5), 4);
    assert_eq!(Board::cycle(-7, -5, 5), 3);

    assert_eq!(Board::cycle(0, 0, 5), 0);
    assert_eq!(Board::cycle(-1, 0, 5), 4);
    assert_eq!(Board::cycle(-2, 0, 5), 3);
    assert_eq!(Board::cycle(2, 0, 5), 2);
    assert_eq!(Board::cycle(5, 0, 5), 0);
    assert_eq!(Board::cycle(6, 0, 5), 1);

    assert_eq!(Board::cycle(5, 5, 6), 5);
    assert_eq!(Board::cycle(6, 5, 6), 5);
    assert_eq!(Board::cycle(4, 5, 6), 5);

    assert_eq!(Board::cycle(-5, -5, -4), -5);
    assert_eq!(Board::cycle(-4, -5, -4), -5);
    assert_eq!(Board::cycle(-6, -5, -4), -5);

}

#[test]
fn test_restricted_board() {
    let mut my_board = Board::new(Some(10), Some(10));

    my_board.born_at(5, 2);
    assert_eq!(my_board.is_alive(-5, 2), true);

    my_board.born_at(0, -7);
    assert_eq!(my_board.is_alive(0, 3), true);
}
