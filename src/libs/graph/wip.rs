use std::{
    fmt, iter,
    ops::{Index, IndexMut},
};
pub struct AdjList(JaggedArray<usize>);
impl AdjList {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> AdjListIter {
        AdjListIter(self.0.iter())
    }
}
impl Index<usize> for AdjList {
    type Output = [usize];
    fn index(&self, i: usize) -> &[usize] {
        &self.0[i]
    }
}
impl IndexMut<usize> for AdjList {
    fn index_mut(&mut self, i: usize) -> &mut [usize] {
        &mut self.0[i]
    }
}
impl fmt::Debug for AdjList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter().enumerate()).finish()
    }
}
pub struct AdjListIter<'a>(Iter<'a, usize>);
impl<'a> Iterator for AdjListIter<'a> {
    type Item = &'a [usize];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
pub struct AdjListBuilder(Builder<usize>);
impl AdjListBuilder {
    pub fn new(n: usize) -> Self {
        Self(Builder::new(n))
    }
    pub fn with_capacity(n: usize, cap: usize) -> Self {
        Self(Builder::with_capacity(n, cap))
    }
    pub fn edge(&mut self, u: usize, v: usize) {
        self.0.push(u, v);
    }
    pub fn bi_edge(&mut self, u: usize, v: usize) {
        self.edge(u, v);
        self.edge(v, u);
    }
    pub fn extend_bi_edges<I: IntoIterator<Item = (usize, usize)>>(&mut self, it: I) {
        self.0.extend(
            it.into_iter()
                .flat_map(|(u, v)| iter::once((u, v)).chain(iter::once((v, u)))),
        );
    }
    pub fn build(self) -> AdjList {
        AdjList(self.0.build())
    }
}
impl Extend<(usize, usize)> for AdjListBuilder {
    fn extend<T: IntoIterator<Item = (usize, usize)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

pub struct JaggedArray<T> {
    heads: Box<[usize]>,
    buf: Box<[T]>,
}
impl<T> JaggedArray<T> {
    pub fn len(&self) -> usize {
        self.heads.len() - 1
    }
    pub fn is_empty(&self) -> bool {
        self.heads.is_empty()
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { a: self, i: 0 }
    }
}
impl<T> Index<usize> for JaggedArray<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &[T] {
        if let Some([l, r, ..]) = self.heads.get(i..) {
            unsafe { self.buf.get_unchecked(*l..*r) }
        } else {
            &[]
        }
    }
}
impl<T> IndexMut<usize> for JaggedArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut [T] {
        if let Some([l, r, ..]) = self.heads.get(i..) {
            unsafe { self.buf.get_unchecked_mut(*l..*r) }
        } else {
            &mut []
        }
    }
}
impl<'a, T> IntoIterator for &'a JaggedArray<T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub struct Iter<'a, T> {
    a: &'a JaggedArray<T>,
    i: usize,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.a.len() {
            let res = &self.a[self.i];
            self.i += 1;
            Some(res)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.a.len() - self.i;
        (rest, Some(rest))
    }
}
pub struct Builder<T> {
    heads: Vec<usize>,
    nodes: Vec<(T, usize)>,
}
impl<T> Builder<T> {
    pub fn new(n: usize) -> Self {
        Self::with_capacity(n, 0)
    }
    pub fn with_capacity(n: usize, cap: usize) -> Self {
        Self {
            heads: vec![!0; n + 1],
            nodes: Vec::with_capacity(cap),
        }
    }
    pub fn push(&mut self, i: usize, x: T) {
        self.nodes.push((x, self.heads[i]));
        self.heads[i] = self.nodes.len() - 1;
    }
    pub fn build(mut self) -> JaggedArray<T> {
        let mut buf_i = self.nodes.len();
        let mut buf = Vec::<T>::with_capacity(buf_i);
        let buf_p = buf.as_mut_ptr();
        *self.heads.last_mut().unwrap() = buf_i;
        unsafe {
            for h in self.heads.iter_mut().rev().skip(1) {
                let mut nodes_i = *h;
                while let Some((x, next)) = self.nodes.get(nodes_i) {
                    buf_i -= 1;
                    buf_p.add(buf_i).copy_from_nonoverlapping(x, 1);
                    nodes_i = *next;
                }
                *h = buf_i;
            }
            self.nodes.set_len(0);
            buf.set_len(buf.capacity());
        }
        JaggedArray {
            heads: self.heads.into(),
            buf: buf.into(),
        }
    }
}
impl<T> Extend<(usize, T)> for Builder<T> {
    fn extend<I: IntoIterator<Item = (usize, T)>>(&mut self, iter: I) {
        for (i, x) in iter {
            self.push(i, x);
        }
    }
}
