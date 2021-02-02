use std::num::NonZeroU8;

pub type Number = NonZeroU8;

pub const N01: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
pub const N02: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
pub const N03: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };
pub const N04: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(4) };
pub const N05: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(5) };
pub const N06: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(6) };
pub const N07: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(7) };
pub const N08: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };
pub const N09: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(9) };
pub const N10: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(10) };
pub const N11: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(11) };
pub const N12: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(12) };
pub const N13: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(13) };
pub const N14: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(14) };
pub const N15: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(15) };
pub const N16: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(16) };
pub const N17: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(17) };
pub const N18: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(18) };
pub const N19: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(19) };
pub const N20: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(20) };

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Square {
    square: [[Option<Number>; 4]; 4],
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    squares: Vec<Square>,
    table: Vec<Number>,
}

pub trait Stack {
    fn pop(&mut self) -> Number;
}

impl Square {
    pub fn put_unchecked(&mut self, row: usize, col: usize, num: Number) -> Option<Number> {
        self.square[row][col].replace(num)
    }

    pub fn put(&mut self, row: usize, col: usize, num: Number) -> Result<Option<Number>, ()> {
        if self.can_put(row, col, num) {
            Ok(self.put_unchecked(row, col, num))
        } else {
            Err(())
        }
    }

    pub fn can_put(&self, row: usize, col: usize, num: Number) -> bool {
        fn ng_p<I: Iterator<Item = Option<Number>>>(it: I, num: Number) -> bool {
            for c in it {
                match c {
                    Some(n) if n <= num => return true,
                    Some(_) => return false,
                    None => (),
                }
            }
            false
        }

        fn ng_m<I: Iterator<Item = Option<Number>>>(it: I, num: Number) -> bool {
            for c in it {
                match c {
                    Some(n) if num <= n => return true,
                    Some(_) => return false,
                    None => (),
                }
            }
            false
        }

        if ng_p(self.square[row][col + 1..4].iter().map(|n| *n), num) {
            return false;
        }
        if ng_m(self.square[row][0..col].iter().rev().map(|n| *n), num) {
            return false;
        }
        if ng_p(self.square[row + 1..4].iter().map(|r| r[col]), num) {
            return false;
        }
        if ng_m(self.square[0..row].iter().rev().map(|r| r[col]), num) {
            return false;
        }
        true
    }

    pub fn count(&self) -> u8 {
        let mut count = 0;
        for row in self.square.iter() {
            for c in row {
                if c.is_some() {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let mut sq = Square::default();
        assert_eq!(sq.put(0, 0, N04), Ok(None));
        assert_eq!(sq.put(1, 1, N10), Ok(None));
        assert_eq!(sq.put(2, 2, N15), Ok(None));
        assert_eq!(sq.put(3, 3, N17), Ok(None));
        assert_eq!(sq.count(), 4);
        assert_eq!(sq.put(2, 3, N16), Ok(None));
        assert_eq!(sq.put(3, 0, N15), Ok(None));
        assert_eq!(sq.put(0, 1, N05), Ok(None));
        assert_eq!(sq.put(1, 0, N10), Err(()));
        assert_eq!(sq.put(3, 3, N20), Ok(Some(N17)));
        assert_eq!(sq.count(), 7);
        assert_eq!(sq.put(3, 2, N17), Ok(None));
        assert_eq!(sq.put(1, 0, N09), Ok(None));
        assert_eq!(sq.put(1, 1, N06), Err(()));
    }
}
