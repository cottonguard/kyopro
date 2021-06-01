pub trait Graph<'a, T: 'a> {
    type Adj: Iterator<Item = (usize, &'a T)>;
    fn adj(&'a self, u: usize) -> Self::Adj;
    fn adj_unlabeled(&'a self, u: usize) -> AdjUnlabeled<Self::Adj> {
        AdjUnlabeled(self.adj(u))
    }
    fn len(&self) -> usize;
}
pub struct AdjUnlabeled<I>(I);
impl<T, I: Iterator<Item = (usize, T)>> Iterator for AdjUnlabeled<I> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(v, _)| v)
    }
}
#[derive(Clone)]
pub struct AdjList<T> {
    heads: Vec<usize>,
    edges: Vec<(usize, T)>,
}
impl<T> AdjList<T> {
    pub fn builder(n: usize) -> AdjListBuilder<T> {
        AdjListBuilder {
            heads: vec![!0; n],
            tails: vec![],
            edges: vec![],
        }
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            g: self,
            u: 0,
        }
    }
    pub fn from_labeled_edges<I: IntoIterator<Item = (usize, usize, T)>>(n: usize, iter: I) -> Self {
        let mut b = Self::builder(n);
        b.extend_labeled(iter);
        b.build()
    }
    pub fn from_labeled_bi_edges<I: IntoIterator<Item = (usize, usize, T)>>(n: usize, iter: I) -> Self
    where
        T: Clone
    {
        let mut b = Self::builder(n);
        b.extend_labeled_bi_edges(iter);
        b.build()
    }
}
impl AdjList<()> {
    pub fn from_edges<I: IntoIterator<Item = (usize, usize)>>(n: usize, iter: I) -> Self {
        Self::from_labeled_edges(n, iter.into_iter().map(|(u, v)| (u, v, ())))
    }
    pub fn from_bi_edges<I: IntoIterator<Item = (usize, usize)>>(n: usize, iter: I) -> Self {
        Self::from_labeled_bi_edges(n, iter.into_iter().map(|(u, v)| (u, v, ())))
    }
}
impl<T> std::ops::Index<usize> for AdjList<T> {
    type Output = [(usize, T)];
    fn index(&self, i: usize) -> &Self::Output {
        &self.edges[self.heads[i]..self.heads[i + 1]]
    }
}
impl<T> std::ops::IndexMut<usize> for AdjList<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.edges[self.heads[i]..self.heads[i + 1]]
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for AdjList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries((0..self.len()).map(|u| (u, &self[u]))).finish()
    }
}
impl<'a, T: 'a> Graph<'a, T> for AdjList<T> {
    type Adj = AdjIter<'a, T>;
    fn adj(&'a self, u: usize) -> Self::Adj {
        AdjIter(self[u].iter())
    }
    fn len(&self) -> usize {
        self.heads.len() - 1
    }
}
pub struct AdjIter<'a, T>(std::slice::Iter<'a, (usize, T)>);
impl<'a, T> Iterator for AdjIter<'a, T> {
    type Item = (usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(v, x)| (*v, x))
    }
}
#[derive(Clone)]
pub struct AdjListBuilder<T> {
    heads: Vec<usize>,
    tails: Vec<usize>,
    edges: Vec<(usize, T)>,
}
impl<T> AdjListBuilder<T> {
    pub fn build(mut self) -> AdjList<T> {
        let mut edges = Vec::<(usize, T)>::with_capacity(self.edges.len());
        let edges_ptr = edges.as_mut_ptr();
        let mut ofs = self.edges.len();
        unsafe {
            for head in self.heads.iter_mut().rev() {
                let mut node = *head;
                while let (Some(&t), Some(e)) = (self.tails.get(node), self.edges.get(node)) {
                    ofs -= 1;
                    edges_ptr.add(ofs).write(std::ptr::read(e));
                    node = t;
                }
                *head = ofs;
            }
            self.edges.set_len(0);
            edges.set_len(edges.capacity());
        }
        self.heads.push(edges.len());
        AdjList {
            heads: self.heads,
            edges
        }
    }
    pub fn edge(&mut self, u: usize, v: usize) {
        self.labeled_edge(u, v, ());
    }
    pub fn bi_edge(&mut self, u: usize, v: usize) {
        self.labeled_bi_edge(u, v, ());
    }
    pub fn labeled_edge(&mut self, u: usize, v: usize, x: T) {
        assert!(u < self.heads.len());
        self.edges.push((v, x));
        self.tails.push(self.heads[u]);
        self.heads[u] = self.tails.len() - 1;
    }
    pub fn labeled_bi_edge(&mut self, u: usize, v: usize, x: T)
    where
        T: Clone,
    {
        self.edge(u, v, x.clone());
        self.edge(v, u, x);
    }
    pub fn extend_labeled<I: IntoIterator<Item = (usize, usize, T)>>(&mut self, iter: I) {
        for (u, v, x) in iter {
            self.edge(u, v, x);
        }
    }
    pub fn extend_labeled_bi_edges<I: IntoIterator<Item = (usize, usize, T)>>(&mut self, iter: I)
    where
        T: Clone,
    {
        for (u, v, x) in iter {
            self.bi_edge(u, v, x);
        }
    }
}
impl AdjListBuilder<()> {
    pub fn extend_bi_edges<I: IntoIterator<Item = (usize, usize)>>(&mut self, iter: I) {
        self.extend_labeled_bi_edges(iter.into_iter().map(|(u, v)| (u, v, ())));
    }
}
impl std::iter::Extend<(usize, usize)> for AdjListBuilder<()> {
    fn extend<I: IntoIterator<Item = (usize, usize)>>(&mut self, iter: I) {
        self.extend_labeled(iter.into_iter().map(|(u, v)| (u, v, ())));
    }
}
