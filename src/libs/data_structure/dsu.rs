pub struct Dsu(Vec<isize>);
impl Dsu {
    pub fn new(n: usize) -> Self {
        Self(vec![-1; n])
    }
    pub fn root(&self, mut u: usize) -> usize {
        while self.0[u] >= 0 {
            u = self.0[u] as usize;
        }
        u
    }
    pub fn is_root(&self, u: usize) -> bool {
        self.0[u] < 0
    }
    pub fn unite(&mut self, u: usize, v: usize) -> (usize, usize) {
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return (ru, ru);
        }
        let (r, c) = if -self.0[ru] >= -self.0[rv] {
            (ru, rv)
        } else {
            (rv, ru)
        };
        self.0[r] += self.0[c];
        self.0[c] = r as isize;
        (r, c)
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
    pub fn size(&self, u: usize) -> usize {
        -self.0[self.root(u)] as usize
    }
}

use std::mem::ManuallyDrop;
pub struct DsuWithData<T> {
    inner: Dsu,
    data: Vec<ManuallyDrop<T>>,
}
impl<T> DsuWithData<T> {
    pub fn unite<F>(&mut self, u: usize, v: usize, mut merge: F) -> (usize, usize)
    where
        F: FnMut(&mut T, T),
    {
        let (r, c) = self.inner.unite(u, v);
        if r != c {
            unsafe {
                let dc = ManuallyDrop::take(&mut self.data[c]);
                merge(&mut self.data[r], dc);
            }
        }
        (r, c)
    }
    pub fn root(&self, u: usize) -> usize {
        self.inner.root(u)
    }
    pub fn is_root(&self, u: usize) -> bool {
        self.inner.is_root(u)
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.inner.is_same(u, v)
    }
    pub fn size(&self, u: usize) -> usize {
        self.inner.size(u)
    }
}
impl<T> std::iter::FromIterator<T> for DsuWithData<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data: Vec<_> = iter.into_iter().map(ManuallyDrop::new).collect();
        Self {
            inner: Dsu::new(data.len()),
            data,
        }
    }
}
impl<T> std::ops::Index<usize> for DsuWithData<T> {
    type Output = T;
    fn index(&self, u: usize) -> &T {
        &self.data[self.root(u)]
    }
}
impl<T> Drop for DsuWithData<T> {
    fn drop(&mut self) {
        for (p, d) in self.inner.0.iter().zip(self.data.drain(..)) {
            if *p < 0 {
                ManuallyDrop::into_inner(d);
            }
        }
    }
}
