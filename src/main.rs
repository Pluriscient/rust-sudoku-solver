use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use crate::sudokutwo::sudoku_api::SudokuApi;

mod sudokutwo;

fn main() -> io::Result<()> {
    let file = File::open("resources/sudoku-easy-1.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut s = match crate::sudokutwo::Sudoku::new(line?) {
            Ok(s) => s,
            Err(msg) => panic!(msg),
        };
        s.solve_once();
    }
    Ok(())
}

//fn main_one() -> io::Result<()> {
//    let file = File::open("resources/top-95.txt")?;
//    let reader = BufReader::new(file);
//    let mut i = 0;
//    let mut solved: Vec<usize> = vec!();
//    let mut unsolved: Vec<usize> = vec!();
//    for line in reader.lines() {
//        let mut s = match Sudoku::new(line?) {
//            Ok(sud) => sud,
//            Err(msg) => panic!(msg)
//        };
//        s.solve();
//        if s.is_solved() {
//            solved.push(i);
//        } else {
//            unsolved.push(i);
//        }
//        println!("Puzzle {} solved?: {}", i, s.is_solved());
//        if !s.is_solved() {
//            println!("Unsolved puzzle state:\n{} ", s)
//        }
////        println!("{}", s.to_string());
//        i += 1;
//    }
//    println!("Solved {} puzzles: {:?}", solved.len(), solved);
//    println!("Could not solve puzzles {:?}", unsolved);
//    Ok(())
//}

// general approach
// 1. We get a sudoku puzzle in a human-readable format as input, with blanks signified as _
// 2. We transform this into a matrix with entries being either Value(Integer), Possible(List[Integer]), or Blank
// 3. We iterate over the matrix to fill in all the possible lists. When there is just one possible we can already fill it in.
// 4. We start using some sudoku methods to eliminate possibile values

//impl<T: std::cmp::PartialEq> Subsettable for Vec<T> {
//    /// Checks whether the given vector's elements are all within the current vector
//    fn contains_all(self, other: Vec<T>) -> bool {
//        for i in other {
//            if !self.contains(&i) {
//                return false;
//            }
//        }
//        true
//    }
//}
