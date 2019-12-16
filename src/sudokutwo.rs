use std::borrow::BorrowMut;
use std::fmt::{Display, Error, Formatter};

use crate::sudokutwo::entry_num::{EntryNum, EntryNumThings};
use crate::sudokutwo::sudoku_api::SudokuApi;

pub mod sudoku_api;
mod entry_num;
mod sudoku_essentials;

enum EntityType {
    Box,
    Col,
    Row,
}

impl Sudoku {
    fn get_sets(sets: Vec<EntryNum>, all_pots: EntryNum, data: &[EntryNum; 81]) -> EntryNum {
//        sets.sort_by_key(|x|x.count_ones());
        let mut res = 0;
        for set in &sets {
            let mut cur = set.clone();
            // if cur subset of res
            if (cur ^ res) & cur == 0 {
                continue;
            }
            while cur != all_pots {
                // remaining == all the sets where the cur has been removed
                // todo check lambda
                let mut remaining: Vec<u16> = sets.iter()
                    .map(|x| (x ^ cur) & x)
                    .collect::<Vec<_>>();
                remaining.sort_by_key(|x| x.count_ones());
                let counted = remaining.iter()
                    .take_while(|x| **x == 0).count();
                if counted == cur.count_ones() as usize {
                    // done
                    res |= cur;
                    break;
                } else if counted > cur.count_ones() as usize {
                    println!("{:?} -> {:?}", cur.get_pos(),
                             remaining.iter().map(|r| r.get_pos()).collect::<Vec<_>>());
                    panic!("something went wrong here...");
                }
                // add the smallest possible to the cur
                cur |= remaining[counted];
            }
        }
        return res;
    }

    /// Get all the entries that are taken, given as a single entrynum that combines them
    fn get_taken(index: usize, to_consider: &Vec<usize>, data: &[u16; 81]) -> EntryNum {
        let mut res: EntryNum = 0;
        let mut sets = vec!();
        for i in to_consider {
            if i == &index {
                continue;
            }
            let en = data[(*i) as usize];
            if en.is_fixed() {
                res |= en;
            } else {
                sets.push(en);
            }
        }
        // now to find the sets
        let largest_missing: EntryNum = sets.iter().fold(0 as u16, |cur, ne| cur | *ne);
        // order by increasing subsets
        let taken_by_sets = Sudoku::get_sets(sets, largest_missing, data);
//        println!("For {}, {:?} were taken by sets", index, taken_by_sets.get_pos());
        res |= taken_by_sets;

        res
    }


    fn eliminate_basic_possibilities(&mut self) -> u32 {
        let mut res = 0;
        for i in 0..9 {
            for j in 0..9 {
                let index = i * 9 + j;
                if self.data[index].is_fixed() {
                    continue;
                }
                let col_nums = Sudoku::get_col_indices(index);
                let row_nums = Sudoku::get_row_indices(index);
                let box_nums = Sudoku::get_block_indices_by_cell_index(index);
                let col_taken = Sudoku::get_taken(index, &col_nums, &self.data);
                let row_taken = Sudoku::get_taken(index, &row_nums, &self.data);
                let box_taken = Sudoku::get_taken(index, &box_nums, &self.data);
                let all_taken = col_taken | row_taken | box_taken;
                res += Sudoku::remove_possibilities(self.data.borrow_mut(), index, all_taken);
                if self.data[index].is_fixed() {
//                    println!("index {} was made fixed", index);
                }
            }
        }
        res
    }

    //    fn find_omissions(&mut self, nums_to_check : Vec<usize>)
    const BLOCK_ONE: [usize; 3] = [0, 1, 2];
    const BLOCK_TWO: [usize; 3] = [3, 4, 5];
    const BLOCK_THREE: [usize; 3] = [6, 7, 8];
    /// Find omissions in the sudoku
    /// The gist of the concept is this:
    /// when pencil marks in a row or column are contained inside a single block,
    /// pencil marks elsewhere in the block can be removed.
    fn eliminate_omissions(&mut self) -> u32 {
        // go through each row & column (just row for now)
        for i in 0..9 {
            let row_nums = (0..9).map(|x| i * 9 + x).collect::<Vec<_>>();
            let col_nums = (0..9).map(|x| x * 9 + i).collect::<Vec<_>>();
            let block_row = i / 3;
            let block_col = i % 3;
            let block_nums = Sudoku::get_block_indices(block_row, block_col);
            self.eliminate_omission(i, row_nums, EntityType::Row);
            self.eliminate_omission(i, col_nums, EntityType::Col);
            self.eliminate_omission(i, block_nums, EntityType::Box);
        }
        0
    }

    fn eliminate_omission(&mut self, rcb: usize, indices: Vec<usize>, t: EntityType) -> u32 {
        let ens = indices.iter().map(|x| self.data[*x]).collect::<Vec<_>>();
        // for each number, check where it is possible
        let mut res = 0;
        for i in 1..=9 {
            let mask = entry_num::to_entry_num(i);
            // get the positions where we are possible (and the related indices)
            let mut pos_locs = ens.iter().enumerate()
                .filter(|(i, x)| **x & mask > 0)
                .collect::<Vec<_>>();

            match pos_locs.len() {
                // fill in an option when it's the only possible thing in the Row|Column|Block)
                1 if !pos_locs[0].1.is_fixed() => self.data.borrow_mut()[indices[pos_locs[0].0]] = mask,
                2 | 3 => { // we need to check whether these are in the same row|column|bloxk
                    // so we can eliminate the rest
                    let actual_indices: Vec<usize> = pos_locs.iter()
                        .map(|(i, _v)| indices[*i]).collect::<Vec<_>>();
                    let first_index = actual_indices[0];
                    let to_remove: Vec<usize> = match t {
                        // we get the indices from which we would want to remove (or none)
                        EntityType::Row if Sudoku::in_same_RCB(&actual_indices, EntityType::Box) =>
                            Sudoku::get_block_indices_by_cell_index(first_index),
                        EntityType::Box if Sudoku::in_same_RCB(&actual_indices, EntityType::Row)
                        => Sudoku::get_row_indices(first_index),
                        EntityType::Box if Sudoku::in_same_RCB(&actual_indices, EntityType::Col) =>
                            Sudoku::get_col_indices(first_index),
                        EntityType::Col if Sudoku::in_same_RCB(&actual_indices, EntityType::Box) =>
                            Sudoku::get_block_indices_by_cell_index(first_index),
                        _ => vec!()
                    };
                    if to_remove.is_empty() { // continue on none
                        continue;
                    }
//                    println!("Removing {:?} at indices {:?}", mask.get_pos(), to_remove);
                    let removal: u32 = to_remove.iter()
                        .filter(|x| !actual_indices.contains(*x))
                        .map(|x| Sudoku::remove_possibilities(self.data.borrow_mut(), *x, mask))
                        .sum();
                    res += removal;
                }
                _ => () // nothing to do in other cases, can't eliminate anything
            }
//            if pos_locs.len() <= 3 { // possible to fit in a box
//                // todo check that they are all in the same box
//                // simple really; have to be contiguous and
//                pos_locs.sort();
//                let mut indices = pos_locs.iter().map(|(index, value)| indices[*index]).collect::<Vec<_>>();
//                // check if we're a block
//                let block_locs = pos_locs.iter().map(|(i, v)| *i).collect::<Vec<_>>();
//                if block_locs == Sudoku::BLOCK_ONE || block_locs == Sudoku::BLOCK_TWO || block_locs == Sudoku::BLOCK_THREE {
//                    res += self.remove_other_locs(rcb, mask, indices, block_locs);
//                }
//            }
        }
        res
    }

    fn in_same_RCB(indices: &Vec<usize>, t: EntityType) -> bool {
        let rcbs = match t {
            EntityType::Row => indices.iter().map(|i| *i / 9).collect::<Vec<_>>(),
            EntityType::Col => indices.iter().map(|i| *i % 9).collect::<Vec<_>>(),
            EntityType::Box => indices.iter().map(|i| Sudoku::get_block_index(*i)).collect::<Vec<_>>()
        };
        let first = rcbs[0];
        rcbs.iter().all(|x| *x == first)
    }


    fn remove_other_locs(&mut self, rcb_index: usize, mask: u16, indices: Vec<usize>, block_locs: Vec<usize>) -> u32 {
        let block_row = rcb_index / 3;
        let block_col = block_locs[0] / 3;
// we need to remove the other possible numbers
        let bloc_indices = Sudoku::get_block_indices(block_row, block_col);
        let removed: u32 = bloc_indices.iter()
            .filter(|x| !indices.contains(x))
            .map(|x| Sudoku::remove_possibilities(self.data.borrow_mut(), *x, mask)).sum();
        return removed;
    }
}


mod sudoku_get_indices;


pub struct Sudoku {
    data: [EntryNum; 81],
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = data_to_string(&self.data);
        write!(f, "{}", content)
    }
}

fn data_to_string(data: &[u16; 81]) -> String {
    let mut s = String::from("");
    //    let s = (0..9).map(|i| (0..9).map(|j| )

    for i in 0..9 {
        for j in 0..9 {
            let n = data[i * 9 + j];
            let pots = n.get_pos();
            let addition = match pots.len() {
                1 => format!("{}", pots[0]),
                _ => format!("{:?}", pots),
            };
            s += &addition;
        }
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod testing {
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::sudokutwo::{data_to_string, Sudoku};
    use crate::sudokutwo::sudoku_api::SudokuApi;

//    const _BASIC_SUDOKU_DATA: [u16; 81] = [0, 0, 3, 0, 2, 0, 6, 0, 0, 9, 0, 0, 3, 0, 5, 0, 0, 1, 0, 0, 1, 8, 0, 6, 4, 0, 0, 0, 0, 8, 1, 0, 2, 9, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 6, 7, 0, 8, 2, 0, 0, 0, 0, 2, 6, 0, 9, 5, 0, 0, 8, 0, 0, 2, 0, 3, 0, 0, 9, 0, 0, 5, 0, 1, 0, 3, 0, 0];

    fn load_sudoku() -> Sudoku {
        Sudoku::new(String::from(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        ))
            .unwrap()
    }

    #[test]
    fn sudoku_ops() {}

    #[test]
    fn import_test() -> Result<(), Box<dyn Error>> {
        let _s = Sudoku::new(String::from(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        ))?;
        Ok(())
    }

    #[test]
    fn solve_once_test() {
        let mut s = load_sudoku();
        println!("Before: \n{}", data_to_string(s.data.borrow()));
        s.solve_once();
        println!("After: \n{}", data_to_string(s.data.borrow()));
    }

    #[test]
    fn solve_all_easy() -> Result<(), Box<dyn Error>> {
        let file = File::open("resources/sudoku-easy-50.txt")?;
        let reader = BufReader::new(file);
        let solved_count = reader.lines().map(|l| l.unwrap()).map(Sudoku::new).map(|x| x.unwrap()).enumerate()
            .map(|(i, mut s)| {
                let before_unfixed = s.count_unfixed();
                while s.solve_once() > 0 {
                    if let Err(msg) = s.is_valid() {
                        panic!(msg);
                    }
                }
                assert_eq!(s.is_valid(), Ok(())); // even if not solved, it should be valid
                if !s.is_solved() {
                    println!("{}", s);
                    println!("{} went from {} unfixed to {} ({})", i, before_unfixed, s.count_unfixed(), before_unfixed - s.count_unfixed());
                }
                s
            })
            .filter(Sudoku::is_solved)
            .count();
        println!("Solved {} out of 50 -> {:.2}%", solved_count, 100 as f32 * solved_count as f32 / 50 as f32);
        println!("DONE ALL 50");
        Ok(())
    }

    #[test]
    fn solve_top_95() -> Result<(), Box<dyn Error>> {
        let file = File::open("resources/top-95.txt")?;
        let reader = BufReader::new(file);
        let solved_count = reader.lines().map(|l| l.unwrap()).map(Sudoku::new).map(|x| x.unwrap()).enumerate()
            .map(|(i, mut s)| {
                let before_unfixed = s.count_unfixed();
                while s.solve_once() > 0 {
                    if let Err(msg) = s.is_valid() {
                        panic!(msg);
                    }
                }
                assert_eq!(s.is_valid(), Ok(())); // even if not solved, it should be valid
                if !s.is_solved() {
                    println!("{} went from {} unfixed to {} ({})", i, before_unfixed, s.count_unfixed(), before_unfixed - s.count_unfixed());
                }
                s
            })
            .filter(Sudoku::is_solved)
            .count();
        println!("Solved {} out of 95 -> {:.2}%", solved_count, 100 as f32 * solved_count as f32 / 95 as f32);
        println!("DONE ALL 95");
        Ok(())
    }

    #[test]
    fn solve() {
        let mut s = load_sudoku();
        while s.solve_once() > 0 {
            if let Err(msg) = s.is_valid() {
                panic!(msg);
            }
            println!("solving... \n{}", data_to_string(s.data.borrow()))
        }
        assert_eq!(s.is_valid(), Ok(()));
        println!("After: \n{}", data_to_string(s.data.borrow()));
    }
}
