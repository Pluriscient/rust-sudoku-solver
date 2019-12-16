use crate::sudokutwo::Sudoku;

impl Sudoku {
    /// Get all the indices of the given block (at the given block index)
    pub(crate) fn get_block_indices_by_cell_index(cell_index: usize) -> Vec<usize> {
        let box_col = (cell_index % 9) / 3;
        let box_row = cell_index / 27;
        return Sudoku::get_block_indices(box_row, box_col);
    }

    pub(crate) fn get_block_index(cell_index: usize) -> usize {
        let box_col = (cell_index % 9) / 3;
        let box_row = cell_index / 27;
        box_row * 3 + box_col
    }

    pub(crate) fn get_block_indices(block_row: usize, block_col: usize) -> Vec<usize> {
        (0..3).flat_map(
            |b_r| (0..3).map(|b_c| (block_row * 3 + b_r) * 9 + (block_col * 3) + b_c).collect::<Vec<_>>()).collect::<Vec<_>>()
    }

    pub(crate) fn get_col_indices(cell_index: usize) -> Vec<usize> {
        (0..9)
            .map(|x| x * 9 + (cell_index % 9))
            //                    .filter(|x| x != &index)
            .collect::<Vec<_>>()
    }

    pub(crate) fn get_row_indices(cell_index: usize) -> Vec<usize> {
        (0..9)
            .map(|x| (cell_index / 9) * 9 + x)
            //                    .filter(|x| x != &index)
            .collect::<Vec<_>>()
    }
}