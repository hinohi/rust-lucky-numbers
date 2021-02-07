use std::{
    collections::HashMap,
    fmt::{self, Write},
    num::NonZeroU8,
};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    turn: usize,
    squares: Vec<Square>,
    table: HashMap<Number, u8>,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Action {
    StackToSquare(usize, usize, Number),
    StackToTable(Number),
    TableToSquare(usize, usize, Number),
}

impl Board {
    pub fn new(n: usize, stack: &mut Vec<Number>) -> Board {
        let mut squares = Vec::with_capacity(n);
        for _ in 0..n {
            let mut sq = Square::default();
            let mut diagonal = (0..4).map(|_| stack.pop().unwrap()).collect::<Vec<_>>();
            diagonal.sort();
            for (i, num) in diagonal.into_iter().enumerate() {
                sq.put_unchecked(i, i, num);
            }
            squares.push(sq);
        }
        Board {
            turn: 0,
            squares,
            table: HashMap::new(),
        }
    }

    fn square_mut(&mut self) -> &mut Square {
        let i = self.turn % self.squares.len();
        self.squares.get_mut(i).unwrap()
    }

    fn square(&self) -> &Square {
        let i = self.turn % self.squares.len();
        self.squares.get(i).unwrap()
    }

    fn take_from_table(&mut self, num: Number) -> Result<(), ()> {
        let c = self.table.entry(num).or_insert(0);
        if *c > 0 {
            *c -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn put_to_table(&mut self, num: Number) {
        *self.table.entry(num).or_insert(0) += 1;
    }

    fn put_to_square_unchecked(&mut self, row: usize, col: usize, num: Number) {
        if let Some(n) = self.square_mut().put_unchecked(row, col, num) {
            self.put_to_table(n)
        }
    }

    fn put_to_square(&mut self, row: usize, col: usize, num: Number) -> Result<(), ()> {
        match self.square_mut().put(row, col, num) {
            Ok(Some(n)) => {
                self.put_to_table(n);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(()) => Err(()),
        }
    }

    pub fn put_unchecked(&mut self, action: Action) {
        match action {
            Action::StackToSquare(row, col, num) => self.put_to_square_unchecked(row, col, num),
            Action::StackToTable(num) => self.put_to_table(num),
            Action::TableToSquare(row, col, num) => {
                self.take_from_table(num).unwrap();
                self.put_to_square_unchecked(row, col, num);
            }
        }
        self.turn += 1;
    }

    pub fn put(&mut self, action: Action) -> Result<(), ()> {
        match action {
            Action::StackToSquare(row, col, num) => self.put_to_square(row, col, num)?,
            Action::StackToTable(num) => self.put_to_table(num),
            Action::TableToSquare(row, col, num) => {
                self.take_from_table(num)?;
                self.put_to_square(row, col, num)?;
            }
        };
        self.turn += 1;
        Ok(())
    }

    pub fn candidates_from_table(&self) -> Vec<(usize, usize, Number)> {
        let mut candidates = Vec::new();
        for (&num, _) in self.table.iter() {
            for row in 0..4 {
                for col in 0..4 {
                    if self.square().can_put(row, col, num) {
                        candidates.push((row, col, num));
                    }
                }
            }
        }
        candidates
    }

    pub fn counts(&self) -> Vec<u8> {
        self.squares.iter().map(|s| s.count()).collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("table:")?;
        let mut tables = self
            .table
            .iter()
            .filter_map(|(n, c)| if *c == 0 { None } else { Some((*n, *c)) })
            .collect::<Vec<_>>();
        tables.sort();
        for (n, c) in tables {
            for _ in 0..c {
                f.write_str(&format!(" {}", n))?;
            }
        }
        f.write_char('\n')?;
        for i in 0..self.squares.len() {
            f.write_str("   A  B  C  D")?;
            if i + 1 != self.squares.len() {
                f.write_str("    ")?;
            }
        }
        f.write_char('\n')?;
        for row in 0..4 {
            for (i, square) in self.squares.iter().enumerate() {
                if i != 0 {
                    f.write_str("   ")?;
                }
                f.write_str(&format!("{}|", row + 1))?;
                for col in 0..4 {
                    match square.square[row][col] {
                        Some(n) => f.write_str(&format!("{:2}|", n))?,
                        None => f.write_str("  |")?,
                    }
                }
            }
            f.write_char('\n')?;
        }
        f.write_str(&format!("next player: {}", self.turn % self.squares.len()))?;
        Ok(())
    }
}

pub fn new_stack(n: usize) -> Vec<Number> {
    let mut stack = Vec::with_capacity(n * 20);
    for _ in 0..n {
        for i in 1..=20 {
            stack.push(NonZeroU8::new(i).unwrap());
        }
    }
    stack
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
