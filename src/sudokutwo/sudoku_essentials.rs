use crate::sudokutwo::Sudoku;
use crate::sudokutwo::entry_num::EntryNum;

impl Sudoku {
    /// Modifies the data given to remove the possibilities given
    pub(crate) fn remove_possibilities(data: &mut [EntryNum; 81], index: usize, to_remove: EntryNum) -> u32 {
        let prev: EntryNum = data[index];
        data[index] = (data[index] ^ to_remove) & data[index];
        prev.count_ones() - data[index].count_ones()
    }

}