pub mod djikstra;
pub mod hld;
pub mod low_link;
pub mod max_flow;
pub mod min_cost_flow;
pub mod scc;
pub mod tsort;

pub mod wip;

use std::{fmt, mem::ManuallyDrop, ops};
pub struct Graph(LabeledGraph<()>);
impl Graph {
    pub fn builder(n: usize) -> GraphBuilder {
        GraphBuilder(LabeledGraph::builder(n))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn edges(&self) -> Edges {
        Edges(self.0.edges())
    }
}
impl ops::Index<usize> for Graph {
    type Output = [usize];
    fn index(&self, u: usize) -> &Self::Output {
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html#structs-with-1-zst-fields
        unsafe { &*(self.0.index(u) as *const _ as *const _) }
    }
}
impl ops::IndexMut<usize> for Graph {
    fn index_mut(&mut self, u: usize) -> &mut Self::Output {
        unsafe { &mut *(self.0.index_mut(u) as *mut _ as *mut _) }
    }
}
impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries((0..self.len()).map(|u| (u, &self[u])))
            .finish()
    }
}
pub struct Edges<'a>(LabeledEdges<'a, ()>);
impl<'a> Iterator for Edges<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(u, v, _)| (u, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
pub struct GraphBuilder(LabeledGraphBuilder<()>);
impl GraphBuilder {
    pub fn edge(&mut self, u: usize, v: usize) {
        self.0.edge(u, v, ());
    }
    pub fn bi_edge(&mut self, u: usize, v: usize) {
        self.0.bi_edge(u, v, ());
    }
    pub fn extend_bi_edges<I: IntoIterator<Item = (usize, usize)>>(&mut self, iter: I) {
        self.0
            .extend_bi_edges(iter.into_iter().map(|(u, v)| (u, v, ())))
    }
    pub fn build(self) -> Graph {
        Graph(self.0.build())
    }
}
impl Extend<(usize, usize)> for GraphBuilder {
    fn extend<I: IntoIterator<Item = (usize, usize)>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().map(|(u, v)| (u, v, ())))
    }
}
pub struct LabeledGraph<T> {
    edges: Box<[(usize, T)]>,
    heads: Box<[usize]>,
}
impl<T> LabeledGraph<T> {
    pub fn builder(n: usize) -> LabeledGraphBuilder<T> {
        LabeledGraphBuilder {
            nodes: Vec::new(),
            heads: vec![!0; n],
        }
    }
    pub fn len(&self) -> usize {
        self.heads.len() - 1
    }
    pub fn edges(&self) -> LabeledEdges<T> {
        LabeledEdges {
            g: self,
            u: 0,
            i: 0,
        }
    }
}
impl<T> ops::Index<usize> for LabeledGraph<T> {
    type Output = [(usize, T)];
    fn index(&self, u: usize) -> &Self::Output {
        &self.edges[self.heads[u]..self.heads[u + 1]]
    }
}
impl<T> ops::IndexMut<usize> for LabeledGraph<T> {
    fn index_mut(&mut self, u: usize) -> &mut Self::Output {
        &mut self.edges[self.heads[u]..self.heads[u + 1]]
    }
}
impl<T: fmt::Debug> fmt::Debug for LabeledGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries((0..self.len()).map(|u| (u, &self[u])))
            .finish()
    }
}
pub struct LabeledEdges<'a, T> {
    g: &'a LabeledGraph<T>,
    u: usize,
    i: usize,
}
impl<'a, T> Iterator for LabeledEdges<'a, T> {
    type Item = (usize, usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let (v, l) = self.g.edges.get(self.i)?;
        while self.g.heads[self.u + 1] == self.i {
            self.u += 1;
        }
        self.i += 1;
        Some((self.u, *v, l))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.g.edges.len();
        (len, Some(len))
    }
}
pub struct LabeledGraphBuilder<T> {
    nodes: Vec<(usize, ManuallyDrop<T>, usize)>,
    heads: Vec<usize>,
}
impl<T> LabeledGraphBuilder<T> {
    pub fn edge(&mut self, u: usize, v: usize, l: T) {
        self.nodes.push((v, ManuallyDrop::new(l), self.heads[u]));
        self.heads[u] = self.nodes.len() - 1;
    }
    pub fn bi_edge(&mut self, u: usize, v: usize, l: T)
    where
        T: Clone,
    {
        self.edge(u, v, l.clone());
        self.edge(v, u, l);
    }
    pub fn extend_bi_edges<I: IntoIterator<Item = (usize, usize, T)>>(&mut self, iter: I)
    where
        T: Clone,
    {
        for (u, v, l) in iter {
            self.bi_edge(u, v, l);
        }
    }
    pub fn build(mut self) -> LabeledGraph<T> {
        let mut edges = Vec::with_capacity(self.nodes.len());
        let mut heads = Vec::with_capacity(self.heads.len() + 1);
        for &(mut h) in &self.heads {
            heads.push(edges.len());
            while let Some((v, l, next)) = self.nodes.get_mut(h) {
                unsafe {
                    edges.push((*v, ManuallyDrop::take(l)));
                }
                h = *next;
            }
        }
        heads.push(edges.len());
        LabeledGraph {
            edges: edges.into(),
            heads: heads.into(),
        }
    }
}
impl<T> Extend<(usize, usize, T)> for LabeledGraphBuilder<T> {
    fn extend<I: IntoIterator<Item = (usize, usize, T)>>(&mut self, iter: I) {
        for (u, v, l) in iter {
            self.edge(u, v, l);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        const N: usize = 10;
        let mut g = Graph::builder(N);
        g.edge(1, 2);
        g.edge(3, 4);
        g.extend(vec![(1, 5), (6, 3), (1, 3)]);
        let g = g.build();
        let mut es: Vec<_> = g.edges().collect();
        es.sort();
        assert_eq!(es, [(1, 2), (1, 3), (1, 5), (3, 4), (6, 3)]);
    }
}
