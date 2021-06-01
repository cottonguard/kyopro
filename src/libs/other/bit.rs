pub fn subsets(set: usize) -> Subsets {
    Subsets {
        set,
        cur: 0,
        end: false,
    }
}
pub struct Subsets {
    set: usize,
    cur: usize,
    end: bool,
}
impl Iterator for Subsets {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.end {
            None
        } else {
            let res = self.cur;
            self.cur = (self.cur | !self.set).wrapping_add(1) & self.set;
            self.end = self.cur == 0;
            Some(res)
        }
    }
}

pub fn bits(n: u64) -> Bits {
    Bits(n)
}
pub struct Bits(u64);
impl Iterator for Bits {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.0 & !self.0 + 1;
            self.0 ^= lsb;
            Some(lsb)
        }
    }
}

pub fn bit_positions(n: u64) -> BitPositions {
    BitPositions(n)
}
pub struct BitPositions(u64);
impl Iterator for BitPositions {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            let res = self.0.trailing_zeros();
            self.0 ^= 1 << res;
            Some(res)
        }
    }
}
