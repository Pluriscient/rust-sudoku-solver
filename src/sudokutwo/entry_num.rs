use std::convert::TryInto;

pub type EntryNum = u16;

pub const ONE: EntryNum = 0b1;
pub const TWO: EntryNum = 0b10;
pub const THREE: EntryNum = 0b100;
pub const FOUR: EntryNum = 0b1000;
pub const FIVE: EntryNum = 0b10000;
pub const SIX: EntryNum = 0b100000;
pub const SEVEN: EntryNum = 0b1000000;
pub const EIGHT: EntryNum = 0b10000000;
pub const NINE: EntryNum = 0b100000000;
pub const ALL: EntryNum = 0b111111111;

//
///// check that the num is fixed
///// ```
///// assert!(is_fixed(0b0010))
///// assert!(!is_fixed(0b001110))
///// ```
//pub fn is_fixed(num: EntryNum) -> bool {
//    num.count_ones() == 1
//}

pub const NUMS: [EntryNum; 9] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub fn to_entry_num(num: u16) -> EntryNum {
    assert!(num > 0 && num < 10);
    NUMS[(num - 1) as usize]
}

pub trait EntryNumThings {
    fn is_fixed(self) -> bool;
    fn get_fixed(self) -> EntryNum;

    fn get_entry_num(num: u16) -> Option<EntryNum>;

    /// Get all the possible numbers
    /// ```rust
    /// assert_eq!(get_pos(8), vec!(8))
    /// ```
    fn get_pos(self) -> Vec<u16>;
}

impl EntryNumThings for EntryNum {
    fn is_fixed(self) -> bool {
        self.count_ones() == 1
    }

    fn get_fixed(self) -> u16 {
        match self.count_ones() {
            1 => (self.trailing_zeros() + 1).try_into().unwrap(),
            _ => panic!("ha"),
        }
    }

    fn get_entry_num(num: u16) -> Option<EntryNum> {
        if num > 0 && num < 10 {
            return Some(NUMS[(num - 1) as usize]);
        }
        None
    }

    fn get_pos(self) -> Vec<u16> {
        let mut n: u16 = 0b1;
        let mut res: Vec<u16> = vec![];
        for _ in 1..=9 {
            if n & self > 0 {
                res.push((n.trailing_zeros() + 1).try_into().unwrap());
            }
            n <<= 1;
        }
        res
    }
}
//
//pub fn get_fixed_num(num: EntryNum) -> u16 {
//    assert!(num.count_ones() == 1);
//    (num.trailing_zeros() + 1).try_into().unwrap()
//}

///// Get all the possible numbers
///// ```rust
///// assert_eq!(get_pos(8), vec!(8))
///// ```
//pub fn get_pos(num: EntryNum) -> Vec<u16> {
//    let mut n: u16 = 0b1;
//    let mut res: Vec<u16> = vec!();
//    for _ in 1..=9 {
//        if n & num > 0 {
//            res.push((n.trailing_zeros() + 1).try_into().unwrap());
//        }
//        n <<= 1;
//    }
//    res
//}

/// Some tests :)
#[cfg(test)]
mod tests {
    use crate::sudokutwo::entry_num;
    use crate::sudokutwo::entry_num::*;

    #[test]
    fn conversion_one() {
        assert_eq!(entry_num::to_entry_num(1), entry_num::ONE);
        assert_eq!(entry_num::to_entry_num(2), entry_num::TWO);
        assert_eq!(entry_num::to_entry_num(5), entry_num::FIVE);
        assert_eq!(entry_num::to_entry_num(9), entry_num::NINE);
    }

    #[test]
    fn conversion_back() {
        assert_eq!(entry_num::ONE.get_fixed(), 1);
    }

    #[test]
    fn conversion_all() {
        for i in 1..=9 {
            assert_eq!(entry_num::to_entry_num(i).get_fixed(), i);
        }
    }

    #[test]
    fn single_pos_convert() {
        for i in 1..=9 {
            assert_eq!(entry_num::to_entry_num(i).get_pos()[0], i)
        }
    }

    #[test]
    fn multi_pos_convert() {
        let possible = (ONE | THREE | FIVE).get_pos();
        let expected: Vec<u16> = vec![1, 3, 5];
        assert_eq!(possible, expected);
    }

    #[test]
    fn test_removal() {
        let possible = ONE | TWO | THREE | NINE;
        assert_eq!(possible.get_pos(), vec!(1, 2, 3, 9));
        let to_remove = TWO | NINE | EIGHT;
        assert_eq!(to_remove.get_pos(), vec!(2, 8, 9));
        let possible: EntryNum = (possible ^ to_remove) & possible;
        assert_eq!(possible.get_pos(), vec!(1, 3));
    }
}
