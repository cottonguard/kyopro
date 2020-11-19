pub struct Graph(LabeledGraph<()>);
impl Graph {
    pub fn builder(n: usize) -> GraphBuilder {
        GraphBuilder(LabeledGraph::builder(n))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl std::ops::Index<usize> for Graph {
    type Output = [usize];
    fn index(&self, u: usize) -> &Self::Output {
        unsafe { std::mem::transmute(self.0.index(u)) }
    }
}
pub struct GraphBuilder(LabeledGraphBuilder<()>);
impl GraphBuilder {
    pub fn edge(&mut self, u: usize, v: usize) {
        self.0.edge(u, v, ());
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
impl<T> std::ops::Index<usize> for LabeledGraph<T> {
    type Output = [(usize, T)];
    fn index(&self, u: usize) -> &Self::Output {
        &self.edges[self.heads[u]..self.heads[u + 1]]
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
