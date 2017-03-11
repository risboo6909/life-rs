mod symvec;

use self::symvec::SymVec;
use ::board::{BoardInternal, Cell, CellIterType};

pub struct SymVecBased {
    cells: SymVec<SymVec<Cell>>
}


impl BoardInternal for SymVecBased {

    fn get_cell(&self, col: isize, row: isize) -> Option<&Cell> {
        unimplemented!()
    }

    fn set_cell(&mut self, col: isize, row: isize, val: Cell) {
        unimplemented!()
    }

    fn ensure_cell(&mut self, col: isize, row: isize) {
        unimplemented!()
    }

    fn rm_cell(&mut self, col: isize, row: isize) {
        unimplemented!()
    }

    fn get_iter<'a>(&'a self) -> Box<Iterator<Item=CellIterType> + 'a> {
        unimplemented!()
    }

}
