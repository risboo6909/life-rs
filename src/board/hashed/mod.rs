use std::collections::HashMap;
use std::collections::hash_map::Iter;

use ::board::{BoardInternal, Cell};

pub struct HashBased {
    cells: HashMap<(isize, isize), Cell>
}

impl BoardInternal for HashBased {

    fn get_cell(&self, col: isize, row: isize) -> Option<&Cell> {
        self.cells.get(&(col, row))
    }

    fn set_cell(&mut self, col: isize, row: isize, val: Cell) {
        self.cells.insert((col, row), val);
    }

    fn rm_cell(&mut self, col: isize, row: isize) {
        self.cells.remove(&(col, row));
    }

    fn get_iter(&self) -> Iter<(isize, isize), Cell> {
        self.cells.iter()
    }

}

pub fn new() -> Box<BoardInternal> {
    Box::new(HashBased {cells: HashMap::new()})
}