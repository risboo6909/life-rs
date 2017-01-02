/// Expandable game board, it expands on demand in all four directions, constraints
/// can be defined to limit maxmimum memory consumption (thus the game board
/// will behave like a torus), otherwise it will expand "forever" consuming memory :]
///
/// ```
/// let mut my_board = Board::new(5, 5);
/// my_board.born_at(20, 20);
/// ```
/// In fact, board of any size can be created and it will expand on demand,
/// therefor it is recommended to initially create smallest board which
/// will contain initial configuration.

use std::cmp::max;

use ::symvec::SymVec;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub col: isize,
    pub row: isize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Cell {
    Empty,
    Occupied
}

pub struct Board {
    cells: SymVec<SymVec<Cell>>,
}

impl Board {

    pub fn new(width: usize, height: usize) -> Board {

        // minimum board size is 4x4
        let cols = max(width, 4);
        let rows = max(height, 4);

        Board {cells: Board::allocate(cols, rows)}

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

        // extend board by any number of cells if needed

        if row >= 0 {
            while self.cells.need_extend_pos(row) {
                self.cells.push_front(SymVec::new());
            }
        } else {
            while self.cells.need_extend_neg(row) {
                self.cells.push_back(SymVec::new());
            }
        }

        if col >= 0 {
            while self.cells[row].need_extend_pos(col) {
                self.cells[row].push_front(Cell::Empty);
            }
        } else {
            while self.cells[row].need_extend_neg(col) {
                self.cells[row].push_back(Cell::Empty);
            }
        }

    }

    pub fn born_at(&mut self, col: isize, row: isize) {
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

        self.cells[row][col] = Cell::Occupied;
    }

    pub fn kill_at(&mut self, col: isize, row: isize) {
        self.cells[row][col] = Cell::Empty;
    }

    pub fn is_alive(&self, col: isize, row: isize) -> bool {
        self.get_cell(col, row) != Cell::Empty
    }

    pub fn get_cell(&self, col: isize, row: isize) -> Cell {
        // if cell is not yet initialized it is considered as free
        if self.cells.is_available(row) && self.cells[row].is_available(col) {
            self.cells[row][col]
        } else {
            Cell::Empty
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

}

impl<'a> IntoIterator for &'a Board {
    type Item = (Coord, bool, bool);
    type IntoIter = BoardIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let row = -(self.cells.len_neg() as isize);
        let col = -(self.cells[row].len_neg() as isize) - 1;

        BoardIntoIterator{board: self, row: row, col: col,
                          cell_iter: Box::new(self.cells[row].into_iter())}
    }

}

pub struct BoardIntoIterator<'a> {
    board: &'a Board,
    row: isize,
    col: isize,
    cell_iter: Box<Iterator<Item=&'a Cell> + 'a>,
}

impl<'a> Iterator for BoardIntoIterator<'a> {

    type Item = (Coord, bool, bool);

    fn next(&mut self) -> Option<(Coord, bool, bool)> {

        match self.cell_iter.next() {

            Some(e) => {

                self.col += 1;
                let coord = Coord {col: self.col, row: self.row};
                return Some((coord, self.board.is_alive(self.col, self.row), false))
            }

            None => {

                // ugly but I don't know how to make it better

                if self.row < self.board.cells.len_pos() as isize - 1 {

                    self.row += 1;
                    self.col = -(self.board.cells[self.row].len_neg() as isize) + 1;

                    self.cell_iter = Box::new(self.board.cells[self.row].into_iter());
                    self.cell_iter.next();

                    return Some((Coord{col: self.col, row: self.row},
                                       self.board.is_alive(self.col, self.row), true))

                } else {
                    return None;
                }

            }

        }

    }
}

impl ToString for Board {
    fn to_string(&self) -> String {

        let mut output = String::new();

        for (_, is_alive, new_line) in self.into_iter() {
            if new_line {
                output.push('\n');
            }
            if is_alive {
                output.push('*');
            } else {
                output.push('.');
            }
        }
        output
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
}

#[test]
fn test_board_iter() {

    let mut my_board = Board::new(5, 5);

    my_board.born_at(0, 0);
    my_board.born_at(1, 1);
    my_board.born_at(2, 2);
    my_board.born_at(3, 3);
    my_board.born_at(4, 4);

    let mut ctr = 0;

    for (_, is_alive, _) in my_board.into_iter() {
        if is_alive {
            ctr += 1;
        }
    }

    assert!(ctr == 5);
}

#[test]
fn test_glyder() {
    let mut my_board = Board::new(5, 5);

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
