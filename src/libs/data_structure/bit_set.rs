use std::ops::{Bound, RangeBounds};
pub struct BitSet(Vec<u64>);
impl BitSet {
    pub fn new(n: usize, b: bool) -> Self {
        let x = if b { !0 } else { 0 };
        Self(vec![x; n / 64 + 1])
    }
    pub fn get(&self, i: usize) -> bool {
        self.0[i / 64] >> i % 64 & 1 == 1
    }
    pub fn set(&mut self, i: usize, b: bool) {
        if b {
            self.0[i / 64] |= 1 << i % 64;
        } else {
            self.0[i / 64] &= !(1 << i % 64);
        }
    }
    pub fn flip<R: RangeBounds<usize>>(&mut self, range: R) {
        let (l, r) = Self::conv_range(range);
        assert!(l <= r);
        self.modify_range(l, r, |v, x| *v ^= x, |v| *v = !*v);
    }
    pub fn fill<R: RangeBounds<usize>>(&mut self, range: R, b: bool) {
        let (l, r) = Self::conv_range(range);
        assert!(l <= r);
        if b {
            self.modify_range(l, r, |v, x| *v |= x, |v| *v = !0);
        } else {
            self.modify_range(l, r, |v, x| *v &= !x, |v| *v = 0);
        }
    }
    fn conv_range<R: RangeBounds<usize>>(range: R) -> (usize, usize) {
        let l = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => i.saturating_sub(1),
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(i) => i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => !0,
        };
        (l, r)
    }
    fn modify_range<F: FnMut(&mut u64, u64), G: FnMut(&mut u64)>(
        &mut self,
        l: usize,
        r: usize,
        mut bound: F,
        mut inter: G,
    ) {
        let ldiv = l / 64;
        let lrem = l % 64;
        let rdiv = r / 64;
        let rrem = r % 64;
        assert!(ldiv < self.0.len());
        if ldiv == rdiv {
            bound(&mut self.0[ldiv], !0 << lrem & !0 >> 64 - rrem);
        } else {
            bound(&mut self.0[ldiv], !0 << lrem);
            let end = rdiv.min(self.0.len());
            for v in &mut self.0[ldiv + 1..end] {
                inter(v);
            }
            if rdiv < self.0.len() && rrem != 0 {
                if let Some(v) = self.0.get_mut(rdiv) {
                    bound(v, !0 >> 64 - rrem);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn bit_set() {
        // todo
    }
}
