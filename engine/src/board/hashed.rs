use std::collections::HashMap;
use std::collections::hash_map::Iter;

use ::board::{BoardInternal, Cell, CellIterType};

pub struct HashBased {
    cells: HashMap<(isize, isize), Cell>
}

pub struct CellsIterator<'a> {
    iter: Iter<'a, (isize, isize), Cell>
}

impl<'a> Iterator for CellsIterator<'a> {

    type Item = CellIterType;

    fn next(&mut self) -> Option<CellIterType> {
        match self.iter.next() {
            Some(e) => {
                let &(col, row) = e.0;
                Some((col, row, *e.1))
            }
            None => None
        }
    }

}

impl<'a> IntoIterator for &'a HashBased {
    type Item = CellIterType;
    type IntoIter = CellsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CellsIterator{iter: self.cells.iter()}
    }
}

impl BoardInternal for HashBased {

    fn get_cell(&self, col: isize, row: isize) -> Option<&Cell> {
        self.cells.get(&(col, row))
    }

    fn set_cell(&mut self, col: isize, row: isize, val: Cell) {
        self.cells.insert((col, row), val);
    }

    fn ensure_cell(&mut self, col: isize, row: isize) {
        if self.get_cell(col, row) == None {
            self.set_cell(col, row, Cell::Empty);
        }
    }

    fn rm_cell(&mut self, col: isize, row: isize) {
        self.cells.remove(&(col, row));
    }

    fn get_iter<'a>(&'a self) -> Box<Iterator<Item=CellIterType> + 'a> {
        Box::new(IntoIterator::into_iter(self))
    }

}

impl HashBased {

    pub fn new() -> Box<BoardInternal> {
        Box::new(HashBased{cells: HashMap::new()})
    }

}
