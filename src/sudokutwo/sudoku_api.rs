use crate::sudokutwo::{entry_num, Sudoku};
use crate::sudokutwo::entry_num::ALL;

pub trait SudokuApi {
    /// Create a new sudoku puzzle entity from a line of text
    fn new(line: String) -> Result<Self, String> where Self: Sized;

    /// Attempt to solve the Sudoku
    /// Returns true if successfully solved, false otherwise
    fn attempt_solve(&mut self) -> bool;

    /// Go through the puzzle once and make as many moves as possible, a move being defined as
    /// 1. Filling in an Entry
    /// 2. Crossing out a possible number
    /// Returns: the number of moves made
    fn solve_once(&mut self) -> usize;

    /// Count the number of non-fixed numbers
    fn count_unfixed(&self) -> usize;

    /// Check that the puzzle is still valid
    fn is_valid(&self) -> Result<(), String>;

    /// Returns true iff. the puzzle is valid & has all numbers filled in
    fn is_solved(&self) -> bool;
}

impl SudokuApi for Sudoku {
    fn new(line: String) -> Result<Self, String> {
        if line.len() != 81 {
            return Err(format!("Line \'{}\' needs to be 81 characters long", line));
        }
        let mut data: [u16; 81] = [0; 81];
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                return Err(format!(
                    "Line \'{}\' contains non-numeric character '{}' at index {}",
                    line, c, i
                ));
            }
            if c != '0' {
                let fixed = c as u16 - '0' as u16;
                let entried = entry_num::to_entry_num(fixed);
                data[i] = entried;
            } else {
                data[i] = ALL;
            }
        }
        Ok(Self { data })
    }


    fn attempt_solve(&mut self) -> bool {
        false
    }

    fn solve_once(&mut self) -> usize {
        // eliminate the numbers based on sets of numbers (of size 1 or more)
        let res = Sudoku::eliminate_basic_possibilities(self);
        //todo forced fills
        let res = res + Sudoku::eliminate_omissions(self);
        res as usize
    }
    fn count_unfixed(&self) -> usize {
        self.data.iter().filter(|en| (**en).count_ones() > 1).count()
    }

    fn is_valid(&self) -> Result<(), String> {
        // check that the puzzle is still valid
        // simple check => no EntryNum that is 0
        if self.data.iter().any(|x| *x == 0) {
            return Err(format!("At position {}, there are no possibilities left", self.data.iter().find(|x| **x == 0).unwrap()));
        }
        // for each row, column, and box, assert that each number is still possible
        for i in 0..9 {
            let row_nums = (0..9).map(|x| i * 9 + x).collect::<Vec<_>>();
            let col_nums = (0..9).map(|x| x * 9 + i).collect::<Vec<_>>();
            let box_row = i / 3;
            let box_col = i % 3;
            let box_nums = (0..3).flat_map(|b_r|
                (0..3).map(|b_c| (box_row * 3 + b_r) * 9 + box_col * 3 + b_c).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for num in 1..=9 {
                let mask = entry_num::to_entry_num(num);
                if !row_nums.iter().any(|x| self.data[*x] & mask > 0) {
                    return Err(format!("{} is not possible in row {}", num, i));
                }
                if !col_nums.iter().any(|x| self.data[*x] & mask > 0) {
                    return Err(format!("{} is not possible in column {}", num, i));
                }
                if !box_nums.iter().any(|x| self.data[*x] & mask > 0) {
                    return Err(format!("{} is not possible in box {}", num, i));
                }
            }
        }
        Ok(())
    }
    fn is_solved(&self) -> bool {
        return self.is_valid().is_ok() && self.data.iter().all(|x| (*x).count_ones() == 1);
    }
}



