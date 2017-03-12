mod symvec;

use self::symvec::SymVec;
use ::board::{BoardInternal, Cell, CellIterType};

pub struct SymVecBased {
    cells: SymVec<SymVec<Cell>>
}

pub struct CellsIterator<'a> {
    row: isize,
    col: isize,
    cells: &'a SymVec<SymVec<Cell>>,
    iter: Box<Iterator<Item=&'a Cell> + 'a>
}

impl<'a> Iterator for CellsIterator<'a> {
    type Item = CellIterType;

    fn next(&mut self) -> Option<CellIterType> {

        match self.iter.next() {
            Some(e) => {
                self.col += 1;
                Some((self.col, self.row, self.cells[self.row][self.col]))
            }

            None => {

                while self.row < (self.cells.len_pos() - 1) as isize {
                    self.row += 1;

                    if self.cells[self.row].len() > 0 {
                        self.col = -(self.cells[self.row].len_neg() as isize);

                        self.iter = Box::new(self.cells[self.row].into_iter());
                        self.iter.next();

                        return Some((self.col, self.row, self.cells[self.row][self.col]));
                    }
                }

                None
            }
        }
    }

}

impl<'a> IntoIterator for &'a SymVecBased {
    type Item = CellIterType;
    type IntoIter = CellsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let row = -(self.cells.len_neg() as isize);
        let col = -(self.cells[row].len_neg() as isize) - 1;

        CellsIterator {
            col: col,
            row: row,
            cells: &self.cells,
            iter: Box::new(self.cells[row].into_iter())
        }
    }

}

impl BoardInternal for SymVecBased {

    fn get_cell(&self, col: isize, row: isize) -> Option<&Cell> {
        if !self.cells.is_available(row) || !self.cells[row].is_available(col) {
            None
        } else {
            Some(&self.cells[row][col])
        }
    }

    fn set_cell(&mut self, col: isize, row: isize, val: Cell) {
        self.cells[row][col] = val;
    }

    fn ensure_cell(&mut self, col: isize, row: isize) {
        // extend board by any number of cells if needed
        // maintain them inside board limits

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

    fn rm_cell(&mut self, col: isize, row: isize) {
        // empty
    }

    fn get_iter<'a>(&'a self) -> Box<Iterator<Item=CellIterType> + 'a> {
        Box::new(IntoIterator::into_iter(self))
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

pub fn new() -> Box<BoardInternal> {
    Box::new(SymVecBased{cells: allocate(2, 2)})
}
