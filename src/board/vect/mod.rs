mod symvec;

use self::symvec::SymVec;
use ::board::{BoardInternal, Cell};

pub struct SymVecBased {
    cells: SymVec<SymVec<Cell>>
}
