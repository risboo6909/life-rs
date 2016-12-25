pub mod board {

    use std::cmp::max;

    pub struct Board {
        cells: Vec<Vec<bool>>,
    }

    impl Board {

        pub fn new(width: usize, height: usize) -> Board {

            // minimum board size if 1x1
            let cols = max(width, 1);
            let rows = max(height, 1);

            let mut tmp: Vec<Vec<bool>> = Vec::new();

            for _ in 0..rows {
                tmp.push(vec![false; cols]);
            }

            Board {cells: tmp}

        }

        pub fn ensure_cell(&mut self, col: usize, row: usize) {

            // extend board by 1 if needed, no need to extend more
            // because we always scan vicinity of radius 1 of any cell

            // TODO: Check negative indices

            if row as isize - self.cells.len() as isize > 0 {
                panic!("Row index is {} but the number of rows is {}",
                        row, self.cells.len());
            }

            if col as isize - self.cells[row].len() as isize > 0 {
                panic!("Col index is {} but the number of cols is {}",
                        col, self.cells[row].len());
            }

            if row >= self.cells.len() {
                self.cells.push(Vec::new());
            }

            if col >= self.cells[row].len() {
                self.cells[row].push(false);
            }

        }

        pub fn set_cell(&mut self, val: bool, col: usize, row: usize) {
            self.ensure_cell(col, row);
            self.cells[row][col] = val;
        }

        pub fn get_cell(&mut self, col: usize, row: usize) -> Option<bool> {
            self.ensure_cell(col, row);
            Some(self.cells[row][col])
        }

    }

}


#[test]
fn test_board_ok() {

    let mut my_board = board::Board::new(5, 5);

    // set some existing cells
    my_board.set_cell(true, 0, 0);
    my_board.set_cell(true, 4, 4);

    // extend board by one cell
    my_board.set_cell(true, 5, 2);

    // test allocated cells
    assert_eq!(my_board.get_cell(0, 0), Some(true));
    assert_eq!(my_board.get_cell(4, 4), Some(true));

    // test previously expanded cell
    assert_eq!(my_board.get_cell(5, 2), Some(true));

    // test existing cell
    assert_eq!(my_board.get_cell(2, 2), Some(false));

    // check extended cell
    assert_eq!(my_board.get_cell(5, 3), Some(false));
}

#[test]
#[should_panic]
fn test_board_panic_extend() {

    let mut my_board = board::Board::new(5, 5);

    // can't extend board more than 1 cell
    my_board.get_cell(3, 6);

}
