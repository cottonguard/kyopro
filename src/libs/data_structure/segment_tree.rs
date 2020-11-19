pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}
pub struct SegmentTree<T>(Box<[T]>);
impl<T: Monoid> SegmentTree<T> {
    pub fn new(n: usize) -> Self {
        Self(std::iter::repeat_with(T::id).take(n << 1).collect())
    }
    pub fn len(&self) -> usize {
        self.0.len() >> 1
    }
    pub fn set(&mut self, i: usize, x: T) {
        let mut i = self.len() + i;
        self.0[i] = x;
        while (i >> 1) > 0 {
            i >>= 1;
            self.0[i] = self.0[i << 1].op(&self.0[(i << 1) + 1]);
        }
    }
    pub fn update(&mut self, i: usize, x: &T) -> &T {
        let j = self.len() + i;
        self.set(i, x.op(&self.0[j]));
        &self.0[j]
    }
    // [l, r)
    pub fn prod(&self, l: usize, r: usize) -> T {
        let mut l = self.len() + l;
        let mut r = self.len() + r;
        assert!(l <= r);
        assert!(r <= self.0.len());
        let mut x = T::id();
        let mut y = T::id();
        while l < r {
            if l & 1 == 1 {
                x = x.op(&self.0[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                y = self.0[r].op(&y);
            }
            l >>= 1;
            r >>= 1;
        }
        x.op(&y)
    }
}
impl<T: Monoid> std::iter::FromIterator<T> for SegmentTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut a: Vec<T> = iter.into_iter().collect();
        let n = a.len();
        a.splice(..0, std::iter::repeat_with(T::id).take(n));
        for i in (1..n).rev() {
            a[i] = a[2 * i].op(&a[2 * i + 1]);
        }
        Self(a.into())
    }
}
impl<T: Monoid, I: std::slice::SliceIndex<[T]>> std::ops::Index<I> for SegmentTree<T> {
    type Output = I::Output;
    fn index(&self, i: I) -> &Self::Output {
        &self.0[self.len()..][i]
    }
}
