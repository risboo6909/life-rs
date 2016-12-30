use std::cmp::max;
use std::collections::HashSet;

use ::symvec::SymVec;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub col: isize,
    pub row: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied
}

pub struct Board {
    cells: SymVec<SymVec<Cell>>,
    occupied: HashSet<Coord>,
}

impl Board {

    pub fn new(width: usize, height: usize) -> Board {

        // minimum board size is 4x4
        let cols = max(width, 4);
        let rows = max(height, 4);

        Board {cells: Board::allocate(cols, rows), occupied: HashSet::new()}

    }

    fn allocate(cols: usize, rows: usize) -> SymVec<SymVec<Cell>> {

        let mut tmp: SymVec<SymVec<Cell>> = SymVec::new();

        for _ in 0..rows {
            let mut col = SymVec::new();
            for _ in 0..cols {
                col.push_front(Cell::Empty);
            }
            tmp.push_front(col);
        }

        tmp
    }

    pub fn ensure_cell(&mut self, col: isize, row: isize) {

        // extend board by 1 if needed, no need to extend more
        // because we always scan vicinity of radius 1 of any cell

        if row >= 0 {
            if self.cells.need_extend_pos(row) {
                self.cells.push_front(SymVec::new());
            }
        } else {
            if self.cells.need_extend_neg(row) {
                self.cells.push_back(SymVec::new());
            }
        }

        if col >= 0 {
            if self.cells[row].need_extend_pos(col) {
                self.cells[row].push_front(Cell::Empty);
            }
        } else {
            if self.cells[row].need_extend_neg(col) {
                self.cells[row].push_back(Cell::Empty);
            }
        }

    }

    pub fn born_at(&mut self, col: isize, row: isize) {
        self.ensure_cell(col, row);
        self.cells[row][col] = Cell::Occupied;

        self.occupied.insert(Coord {col: col, row: row});
    }

    pub fn kill_at(&mut self, col: isize, row: isize) {
        self.ensure_cell(col, row);
        self.cells[row][col] = Cell::Empty;

        self.occupied.remove(&Coord {col: col, row: row});
    }

    pub fn get_cell(&mut self, col: isize, row: isize) -> Cell {
        self.ensure_cell(col, row);
        self.cells[row][col]
    }

    pub fn get_vicinity(&mut self, col: isize, row: isize) -> Vec<Cell> {

        // get contents of 8 neighbours of a given cell

        let neighbours = vec![
            self.get_cell(col - 1, row),
            self.get_cell(col - 1, row - 1),
            self.get_cell(col, row - 1),
            self.get_cell(col + 1, row - 1),
            self.get_cell(col + 1, row),
            self.get_cell(col + 1, row + 1),
            self.get_cell(col, row + 1),
            self.get_cell(col - 1, row +1),
        ];

        neighbours
    }

    pub fn get_occupied<'a>(&'a self) -> Box<Iterator<Item=&'a Coord> + 'a> {
        Box::new(self.occupied.iter())
    }
}


#[test]
fn test_board_ok() {

    use std::collections::HashSet;

    let mut my_board = Board::new(5, 5);

    // set some existing cells
    my_board.born_at(0, 0);
    my_board.born_at(4, 4);

    // extend board by one cell
    my_board.born_at(5, 2);

    // test allocated cells
    assert_eq!(my_board.get_cell(0, 0), Cell::Occupied);
    assert_eq!(my_board.get_cell(4, 4), Cell::Occupied);

    // test previously expanded cell
    assert_eq!(my_board.get_cell(5, 2), Cell::Occupied);

    // test existing cell
    assert_eq!(my_board.get_cell(2, 2), Cell::Empty);

    // check extended cell
    assert_eq!(my_board.get_cell(5, 3), Cell::Empty);

    my_board.kill_at(0, 0);
    assert_eq!(my_board.get_cell(0, 0), Cell::Empty);

    let mut expected: HashSet<Coord> = HashSet::new();

    expected.insert(Coord{col: 5, row: 2});
    expected.insert(Coord{col: 4, row: 4});

    let tmp = my_board.get_occupied().collect::<Vec<&Coord>>();

    assert_eq!(tmp.contains(&&Coord{col: 5, row: 2}), true);
    assert_eq!(tmp.contains(&&Coord{col: 4, row: 4}), true);
    assert_eq!(tmp.len(), 2);

}

//
//#[test]
//#[should_panic]
//fn test_board_panic_extend() {
//
//    let mut my_board = Board::new(5, 5);
//
//    // can't extend board more than 1 cell
//    my_board.get_cell(3, 6);
//
//}
