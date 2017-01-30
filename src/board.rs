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

pub struct CellDesc {
    pub coord: Coord,
    pub is_alive: bool,
    pub new_line: bool,
}

pub struct Board {
    cells: SymVec<SymVec<Cell>>,
    half_width: Option<isize>,
    half_height: Option<isize>,
}

impl Board {

    pub fn new(width: Option<usize>, height: Option<usize>) -> Board {
        // initially we allocate 2x2 board and extend it on demand
        Board { cells: Board::allocate(2, 2),
                half_width: width.map(|x| (x / 2) as isize),
                half_height: height.map(|x| (x / 2) as isize),
              }
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

        let col = Board::bound_coordinate(self.half_width, col);
        let row = Board::bound_coordinate(self.half_height, row);

        (col, row)

    }

    pub fn ensure_cell(&mut self, col: isize, row: isize) {

        // extend board by any number of cells if needed
        // maintain them inside board limits

        let (col, row) = self.constrain_board(col, row);

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

        let (col, row) = self.constrain_board(col, row);

        self.cells[row][col] = Cell::Occupied;

    }

    pub fn kill_at(&mut self, col: isize, row: isize) {
        let (col, row) = self.constrain_board(col, row);
        self.cells[row][col] = Cell::Empty;
    }

    pub fn is_alive(&self, col: isize, row: isize) -> bool {
        self.get_cell(col, row) != Cell::Empty
    }

    pub fn get_cell(&self, col: isize, row: isize) -> Cell {

        // if cell is not yet initialized it is considered as free

        let (col, row) = self.constrain_board(col, row);
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

    type Item = CellDesc;
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

    type Item = CellDesc;

    fn next(&mut self) -> Option<CellDesc> {

        match self.cell_iter.next() {

            Some(e) => {

                self.col += 1;
                let coord = Coord{col: self.col, row: self.row};
                Some(CellDesc { coord: coord,
                                is_alive: self.board.is_alive(self.col, self.row),
                                new_line: false })
            }

            None => {

                // ugly but I don't know how to make it better

                if self.row < self.board.cells.len_pos() as isize - 1 {

                    self.row += 1;
                    self.col = -(self.board.cells[self.row].len_neg() as isize);

                    self.cell_iter = Box::new(self.board.cells[self.row].into_iter());
                    self.cell_iter.next();

                    Some(CellDesc { coord: Coord{col: self.col, row: self.row },
                                    is_alive: self.board.is_alive(self.col, self.row),
                                    new_line: true })
                } else {
                    None
                }

            }

        }

    }
}

impl ToString for Board {
    fn to_string(&self) -> String {

        let mut output = String::new();

        for CellDesc { coord, is_alive, new_line } in self.into_iter() {
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

    let mut my_board = Board::new(Some(10), Some(10));

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
