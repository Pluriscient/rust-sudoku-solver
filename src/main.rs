use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::ops::Div;
use std::rc::Rc;

use crate::Entry::{Fixed, Possible};

fn main() -> io::Result<()> {

    let file = File::open("resources/top-95.txt")?;
    let reader = BufReader::new(file);
    let mut i = 0;
    let mut solved: Vec<usize> = vec!();
    let mut unsolved: Vec<usize> = vec!();
    for line in reader.lines() {
        let mut s = match Sudoku::new(line?) {
            Ok(sud) => sud,
            Err(msg) => panic!(msg)
        };
//        println!("{}", s.to_string());
        s.solve();
        if s.is_solved() {
            solved.push(i);
        } else {
            unsolved.push(i);
        }
        println!("Puzzle {} solved?: {}", i, s.is_solved());
        if !s.is_solved() {
            println!("Unsolved puzzle state:\n{} ", s)
        }
//        println!("{}", s.to_string());
        i += 1;
    }
    println!("Solved {} puzzles: {:?}", solved.len(), solved);
    println!("Could not solve puzzles {:?}", unsolved);
    Ok(())

    // so here we read in the puzzles and throw them to our solver
}

// general approach
// 1. We get a sudoku puzzle in a human-readable format as input, with blanks signified as _
// 2. We transform this into a matrix with entries being either Value(Integer), Possible(List[Integer]), or Blank
// 3. We iterate over the matrix to fill in all the possible lists. When there is just one possible we can already fill it in.


#[derive(Clone)]
enum Entry {
    Possible(RefCell<Vec<u32>>),
    Fixed(u32),
}

trait Subsettable {
    fn contains_all(self, other: Self) -> bool;
}


impl<T: std::cmp::PartialEq> Subsettable for Vec<T> {
    /// Checks whether the given vector's elements are all within the current vector
    fn contains_all(self, other: Vec<T>) -> bool {
        for i in other {
            if !self.contains(&i) {
                return false;
            }
        }
        true
    }
}

fn compare_sets<'a>(this: &'a [u32], other: &'a [u32]) -> Vec<u32> {
    let mut diff = vec!();
    for el in other {
        if !this.contains(el) {
            diff.push(*el);
        }
    }
    diff
}


pub struct Sudoku {
    data: Vec<Entry> // todo maybe change this to [[Entry; 9]; 9]
}

impl Sudoku {
    pub fn is_solved(&self) -> bool {
        self.data.iter().all(|x| match x {
            Fixed(_) => true,
            _ => false
        })
    }

    pub fn new(line: String) -> Result<Self, String> {
        if line.len() != 81 {
            return Err(format!("Line \'{}\' needs to be 81 characters long", line));
        }
        let mut data: Vec<Entry> = vec!();
        for t in line.chars().enumerate() {
            let (i, c) = t;
            if !c.is_numeric() {
                return Err(format!("Line \'{}\' contains non-numeric character '{}' at index {}", line, c, i));
            }
            if c != '0' {
                data.push(Fixed(c as u32 - '0' as u32));
            } else {
                data.push(Possible(RefCell::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9))));
            }
        }
        Ok(Self { data })
    }

    /// Get the indices of the box surrounding the entry at the given location
    fn get_box_indices(index: usize) -> [usize; 9] {
        let row = index.div(9);
        let column = index % 9;
        let box_row = row - (row % 3);
        let box_col = column - (column % 3);
        let _box_i = box_row + column / 3;
        let mut res = [0; 9];
        for r in 0..3 {
            for c in 0..3 {
                res[r * 3 + c] = 9 * (box_row + r) + box_col + c;
            }
        }
        res
    }

    fn get_single_set(list: Vec<u32>, size: usize, lists: &Vec<Vec<u32>>) -> Vec<u32> {
        let mut cur_list = list.to_owned().to_vec();
        while cur_list.len() < size {
            let mut mapped = lists.iter().map(|x| compare_sets(cur_list.borrow(), x))
                .collect::<Vec<_>>();
            mapped.sort_by_key(|x| x.len());
            let found = mapped.iter().take_while(|l| l.len() == 0).count();
            if found >= cur_list.len() {
                return cur_list;
            }
            let extension = mapped.iter()
                .skip_while(|l| l.len() == 0)
                .nth(0).expect("No lists left add, you're not stopping in time");
            cur_list.extend(extension.iter().map(|x| *x));
        }
        return vec!();
    }


    /// get all the things taken by sets
    fn get_sets(lists: Vec<Vec<u32>>, index: usize) -> Vec<u32> {
        let size = lists.len();
        let all_things = lists.iter().map(|l| Sudoku::get_single_set(l.to_vec(), size, &lists)).collect::<Vec<_>>();
//        println!("All things: {:?}", all_things.join(&0));
        let mut r: Vec<_> = all_things.join(&0);
        r.sort();
        r.dedup_by(|x, y| x == y);
        if r.len() > 0 {
            r.remove(0);
        }
//        if r.len() > 0 {
//            println!("at index {}, {:?}", index, r);
//        }
        r
    }


    fn get_group_taken(&self, index: usize, indices: [usize; 9]) -> Vec<u32> {
        let mut taken: Vec<u32> = vec!();
        let mut lists = vec!();
        for di in indices.iter() {
            let i = *di;
            if i == index {
                continue;
            }
            match &self.data[i] {
                Fixed(n) => taken.push(*n), // Just one value here, easy to add to the list
                Possible(list) => lists.push(list.borrow().to_vec()) // gonna do subset comparisons here
            }
        }
        let taken_by_sets = Sudoku::get_sets(lists, index);
//        if taken_by_sets.len() > 0 {
//            println!("For indices {:?}", indices)
//        }
        [taken, taken_by_sets].join(&0) // todo not sure if 0 will have bad effects, and this looks clean
    }


    fn get_row_indices(index: usize) -> [usize; 9] {
        let row = index / 9;
        let mut res = [0; 9];
        for i in 0..9 {
            res[i] = row * 9 + i;
        }
        res
    }

    fn get_col_indices(index: usize) -> [usize; 9] {
        let column = index % 9;
        let mut res = [0; 9];
        for i in 0..9 {
            res[i] = i * 9 + column;
        }
        res
    }

    /// Removes the possibilities that are fixed in the given indices
    fn remove_fixed(&self, remaining: &RefCell<Vec<u32>>, indices: Vec<usize>) {
        let mut found: Vec<u32> = vec!();
        for i in indices {
            match self.data[i] {
                Fixed(n) => found.push(n),
                _ => ()
            }
        }
        found.sort();
        found.dedup_by(|a, b| a == b);
        remaining.borrow_mut().retain(|x| {
            !(found.contains(x))
        });
    }


    fn check_possibilities(&self, index: usize, remaining: RefCell<Vec<u32>>) -> Vec<u32> {
        let row_indices = Sudoku::get_row_indices(index);
        let col_indices = Sudoku::get_col_indices(index);
        let box_indices = Sudoku::get_box_indices(index);
        let row_taken = self.get_group_taken(index, row_indices);
        let col_taken = self.get_group_taken(index, col_indices);
        let box_taken = self.get_group_taken(index, box_indices);

        let mut all_taken = row_taken;
        all_taken.extend(col_taken);
        all_taken.extend(box_taken);
        all_taken.sort();
        all_taken.dedup_by(|x, y| x == y);

        remaining.borrow_mut().retain(|x| !all_taken.contains(x));

        if let Some(i) = self.check_forced(row_indices, index) {
            return vec!(i);
        } else if let Some(i) = self.check_forced(col_indices, index) {
            return vec!(i);
        } else if let Some(i) = self.check_forced(box_indices, index)
        {
            return vec!(i);
        } else {
            return remaining.borrow().to_vec();
        }
    }

    ///Check for moves that we are 'forced to make', ie. when a number can only be placed at that one place
    fn check_forced(&self, indices: [usize; 9], index: usize) -> Option<u32> {
        let mut forced = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9);
        let mut found = vec!();
        for i in indices.iter() {
            if *i == index { continue; }
            match &self.data[*i] {
                Possible(ls) => found.extend_from_slice(ls.borrow().as_slice()),
                Fixed(n) => found.push(*n),
            }
        }
        forced.retain(|x| !found.contains(x));
        match forced.len() {
            0 => None,
            1 => Some(forced[0]),
            _ => panic!("Can't be forced to place 2 numbers...")
        }
    }


    ///Step 3: solve the actual puzzle
    /// todo horrible how references attacked me here, perhaps I should submit this code for some review
    pub fn solve(&mut self) {
        let mut changes = 0;
        let mut new_data: Vec<Entry> = vec!(); // todo optimize this
        for (index, entry) in self.data.iter().enumerate() { // enumerate over each entry in the data
            let new_entry = match entry {
                Fixed(n) => Fixed(*n),
                Entry::Possible(rem) => {
                    let new_possibilities = self.check_possibilities(index, rem.clone());
                    if new_possibilities.len() == 1 {
                        changes += 1;
                        Entry::Fixed(*new_possibilities.first().unwrap())
                    } else {
                        if new_possibilities.len() < rem.borrow().len() {
                            changes += 1;
                        }
                        rem.replace(new_possibilities);
                        Entry::Possible(rem.clone())
                    }
                }
            };
            new_data.push(new_entry);
        }
        self.data = new_data;
        if changes > 0 { // if we're getting progress...
//            println!("{}", self);
            self.solve();
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = match self {
            Entry::Possible(x) => String::from("X"), // format!("{:?}", x.borrow()), //String::from("X"),
            Fixed(n) => n.to_string()
        };
        write!(f, "{}", content)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::from("Sudoku:\n");
        for i in 0..9 {
            for j in 0..9 {
                let index = i * 9 + j;
                if j % 3 == 0 && j > 0 {
                    content += "|";
                }
                content += &self.data[index].to_string();
            }
            content += &"\n";
        }

        write!(f, "{}", content)
    }
}