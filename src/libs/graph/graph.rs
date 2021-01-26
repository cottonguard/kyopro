use std::ops;
pub struct Graph(LabeledGraph<()>);
impl Graph {
    pub fn builder(n: usize) -> GraphBuilder {
        GraphBuilder(LabeledGraph::builder(n))
    }
    pub fn len(&self) -> usize {
        self.0.len()
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
pub struct GraphBuilder(LabeledGraphBuilder<()>);
impl GraphBuilder {
    pub fn edge(&mut self, u: usize, v: usize) {
        self.0.edge(u, v, ());
    }
    pub fn bi_edge(&mut self, u: usize, v: usize) {
        self.0.bi_edge(u, v, ());
    }
    pub fn build(&mut self) -> Graph {
        Graph(self.0.build())
    }
}
pub struct LabeledGraph<T> {
    edges: Box<[(usize, T)]>,
    heads: Box<[usize]>,
}
impl<T: Clone> LabeledGraph<T> {
    pub fn builder(n: usize) -> LabeledGraphBuilder<T> {
        LabeledGraphBuilder {
            nodes: Vec::new(),
            heads: vec![!0; n],
        }
    }
    pub fn len(&self) -> usize {
        self.heads.len() - 1
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
pub struct LabeledGraphBuilder<T> {
    nodes: Vec<((usize, T), usize)>,
    heads: Vec<usize>,
}
impl<T: Clone> LabeledGraphBuilder<T> {
    pub fn edge(&mut self, u: usize, v: usize, l: T) {
        self.nodes.push(((v, l), self.heads[u]));
        self.heads[u] = self.nodes.len() - 1;
    }
    pub fn bi_edge(&mut self, u: usize, v: usize, l: T) {
        self.edge(u, v, l.clone());
        self.edge(v, u, l);
    }
    pub fn build(&mut self) -> LabeledGraph<T> {
        let mut edges = Vec::with_capacity(self.nodes.len());
        let mut heads = Vec::with_capacity(self.heads.len() + 1);
        for &(mut h) in &self.heads {
            heads.push(edges.len());
            while let Some((e, next)) = self.nodes.get(h) {
                edges.push(e.clone());
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

impl Graph {
    pub fn edges(&self) -> Edges {
        Edges(self.0.edges())
    }
}
pub struct Edges<'a>(LabeledEdges<'a, ()>);
impl<'a> Iterator for Edges<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(u, v, _)| (u, v))
    }
}
impl<T> LabeledGraph<T> {
    pub fn edges(&self) -> LabeledEdges<T> {
        LabeledEdges {
            g: self,
            u: 0,
            i: 0,
        }
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
        self.g.edges.get(self.i).map(|(v, l)| {
            while self.g.heads[self.u + 1] == self.i {
                self.u += 1;
            }
            let e = (self.u, *v, l);
            self.i += 1;
            e
        })
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
        g.edge(1, 5);
        g.edge(6, 3);
        g.edge(1, 3);
        let g = g.build();
        let mut es: Vec<_> = g.edges().collect();
        es.sort();
        assert_eq!(es, [(1, 2), (1, 3), (1, 5), (3, 4), (6, 3)]);
    }
}
