/// Represents the game board. Only cells that are not empty and their
/// 8 neighbours are saved in memory for proper simulation.
///
/// Example usage:
/// ```
/// let mut my_board = Board::new(Some(30), Some(30));
/// my_board.born_at(20, 20);
/// ```
///

pub mod vect;
pub mod hashed;


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

pub type CellIterType = (isize, isize, Cell);

pub trait BoardInternal {
    fn get_cell(&self, col: isize, row: isize) -> Option<&Cell>;
    fn set_cell(&mut self, col: isize, row: isize, val: Cell);
    fn ensure_cell(&mut self, col: isize, row: isize);
    fn rm_cell(&mut self, col: isize, row: isize);

    fn get_iter<'a>(&'a self) -> Box<Iterator<Item=CellIterType> + 'a>;
}

pub struct Board<'a> {
    cells: Box<BoardInternal + 'a>,

    population: usize,

    rows: Option<usize>,
    cols: Option<usize>,

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
fn bound_coordinate(left: isize, right: isize, coord: isize) -> isize {
    if coord < left || coord >= right {
        cycle(coord, left, right)
    } else { coord }
}

impl<'a> Board<'a> {

    pub fn new(cells: Box<BoardInternal>, width: Option<usize>, height: Option<usize>) -> Board<'a> {
        Board {

            cells: cells,
            population: 0,

            cols: width,
            rows: height,

        }
    }

    #[inline]
    fn constrain_board(&self, col: isize, row: isize) -> (isize, isize) {

        // ensure cell coordinates lie inside limits

        let mut new_col = col;
        let mut new_row = row;

        let mut left: usize;
        let mut right: usize;

        if let Some(cols) = self.cols {
            if cols % 2 == 0 {
                left = cols / 2;
                right = left;
            } else {
                left = (cols - 1) / 2;
                right = left + 1
            }
            new_col = bound_coordinate(-(left as isize), right as isize, col);
        }

        if let Some(rows) = self.rows {
            if rows % 2 == 0 {
                left = rows / 2;
                right = left;
            } else {
                left = (rows - 1) / 2;
                right = left + 1
            }
            new_row = bound_coordinate(-(left as isize), right as isize, row)
        }

        (new_col, new_row)
    }

    fn ensure_cell(&mut self, col: isize, row: isize) {
        let (col, row) = self.constrain_board(col, row);
        self.cells.ensure_cell(col, row);
    }

    pub fn born_at_gen(&mut self, col: isize, row: isize, gen: usize) {
        if !self.is_alive(col, row) {

            self.ensure_cell(col, row);

            // we must allocate 8 cells around current cell because
            // new species can potentially born there, so we
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
            self.population += 1;
            self.cells.set_cell(col, row, Cell::Occupied { gen: gen });
        }
    }

    pub fn born_at(&mut self, col: isize, row: isize) {
        self.born_at_gen(col, row, 1);
    }

    #[inline]
    pub fn kill_at(&mut self, col: isize, row: isize) {
        let (col, row) = self.constrain_board(col, row);
        self.population -= 1;
        self.cells.rm_cell(col, row);
    }

    #[inline]
    pub fn is_alive(&self, col: isize, row: isize) -> bool {
        self.get_cell(col, row) != Cell::Empty
    }

    pub fn get_cell(&self, col: isize, row: isize) -> Cell {
        // if cell is not yet initialized it is considered as free
        let (col, row) = self.constrain_board(col, row);

        match self.cells.get_cell(col, row) {
            Some(x) => {
                match x {
                   &Cell::Occupied {gen} => *x,
                   _ => Cell::Empty,
                }
            }
            None => Cell::Empty
        }
    }

    pub fn get_cell_gen(&self, col: isize, row: isize) -> usize {
        match self.get_cell(col, row) {
            Cell::Occupied { gen } => gen,
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

    #[inline]
    pub fn get_population(&self) -> usize {
        self.population
    }
}

impl<'a> IntoIterator for &'a Board<'a> {
    type Item = CellDesc;
    type IntoIter = BoardIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIntoIterator {
            board: &self,
            cell_iter: Box::new(self.cells.get_iter())
        }
    }
}

pub struct BoardIntoIterator<'a> {
    board: &'a Board<'a>,
    cell_iter: Box<Iterator<Item=CellIterType> + 'a>
}

impl<'a> Iterator for BoardIntoIterator<'a> {
    type Item = CellDesc;

    fn next(&mut self) -> Option<CellDesc> {

        match self.cell_iter.next() {

            Some(e) => {

                let (col, row, cell) = e;

                let gen = match cell {
                    Cell::Occupied { gen } => gen,
                    Cell::Empty => 0
                };

                Some(CellDesc {
                    coord: Coord { col: col, row: row },
                    gen: gen,
                    is_alive: self.board.is_alive(col, row),
                    new_line: false
                })
            }

            None => None
        }
    }
}


#[test]
fn test_board_ok() {
    let mut my_board = Board::new(new_hashed(), Some(10), Some(10));

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
    let mut my_board = Board::new(new_hashed(), Some(10), Some(10));

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
    let mut my_board = Board::new(new_hashed(), Some(10), Some(10));

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
    assert_eq!(cycle(0, -5, 5), 0);
    assert_eq!(cycle(-5, -5, 5), -5);
    assert_eq!(cycle(5, -5, 5), -5);
    assert_eq!(cycle(6, -5, 5), -4);
    assert_eq!(cycle(-6, -5, 5), 4);
    assert_eq!(cycle(-7, -5, 5), 3);

    assert_eq!(cycle(0, 0, 5), 0);
    assert_eq!(cycle(-1, 0, 5), 4);
    assert_eq!(cycle(-2, 0, 5), 3);
    assert_eq!(cycle(2, 0, 5), 2);
    assert_eq!(cycle(5, 0, 5), 0);
    assert_eq!(cycle(6, 0, 5), 1);

    assert_eq!(cycle(5, 5, 6), 5);
    assert_eq!(cycle(6, 5, 6), 5);
    assert_eq!(cycle(4, 5, 6), 5);

    assert_eq!(cycle(-5, -5, -4), -5);
    assert_eq!(cycle(-4, -5, -4), -5);
    assert_eq!(cycle(-6, -5, -4), -5);
}

#[test]
fn test_restricted_board() {
    let mut my_board = Board::new(new_hashed(), Some(10), Some(10));

    my_board.born_at(5, 2);
    assert_eq!(my_board.is_alive(-5, 2), true);

    my_board.born_at(0, -7);
    assert_eq!(my_board.is_alive(0, 3), true);
}
