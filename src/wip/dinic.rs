pub struct Dinic {
    g: LabeledGraphBuilder<Node>,
    outdeg: Vec<usize>,
}
#[derive(Clone, Copy, Debug)]
struct Node {
    rev_i: usize,
    cap: u64,
}
impl Dinic {
    pub fn new(n: usize) -> Self {
        Self {
            g: LabeledGraph::builder(n),
            outdeg: vec![0; n],
        }
    }
    pub fn edge(&mut self, u: usize, v: usize, cap: u64) {
        if u == v {
            return;
        }
        self.g.edge(
            u,
            v,
            Node {
                rev_i: self.outdeg[v],
                cap,
            },
        );
        self.g.edge(
            v,
            u,
            Node {
                rev_i: self.outdeg[u],
                cap: 0,
            },
        );
        self.outdeg[u] += 1;
        self.outdeg[v] += 1;
    }
    pub fn run(self, s: usize, t: usize) -> u64 {
        let mut g = self.g.build();
        let mut que = VecDeque::new();
        let mut dist = vec![0u32; g.len()];
        let mut stk = vec![];
        let mut cur = vec![0; g.len()];
        let mut sum = 0;
        loop {
            for d in &mut dist {
                *d = !0;
            }
            dist[s] = 0;
            que.push_back(s);
            while let Some(u) = que.pop_front() {
                let du = dist[u];
                for &(v, Node { cap, .. }) in &g[u] {
                    if cap > 0 && dist[v] == !0 {
                        dist[v] = du + 1;
                        que.push_back(v);
                    }
                }
            }
            if dist[t] == !0 {
                break;
            }
            for c in &mut cur {
                *c = 0;
            }
            loop {
                stk.clear();
                stk.push(s);
                'outer: while let Some(&u) = stk.last() {
                    if u == t {
                        break;
                    }
                    while let Some(&(v, Node { cap, .. })) = g[u].get(cur[u]) {
                        if cap > 0 && dist[u] + 1 == dist[v] {
                            stk.push(v);
                            continue 'outer;
                        }
                        cur[u] += 1;
                    }
                    dist[u] = !0;
                    stk.pop();
                }
                if let Some(add) = stk[..stk.len().saturating_sub(1)]
                    .iter()
                    .map(|&u| g[u][cur[u]].1.cap)
                    .min()
                {
                    sum += add;
                    for &u in &stk[..stk.len() - 1] {
                        let (v, Node { ref mut cap, rev_i }) = g[u][cur[u]];
                        *cap -= add;
                        g[v][rev_i].1.cap += add;
                    }
                } else {
                    break;
                }
            }
        }
        sum
    }
}

use std::{mem::ManuallyDrop, ops};
pub struct LabeledGraph<T> {
    edges: Box<[(usize, T)]>,
    heads: Box<[usize]>,
}
impl<T> LabeledGraph<T> {
    pub fn builder(n: usize) -> LabeledGraphBuilder<T> {
        LabeledGraphBuilder {
            nodes: Vec::new(),
            heads: vec![!0; n + 1],
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
    nodes: Vec<((usize, T), usize)>,
    heads: Vec<usize>,
}
impl<T> LabeledGraphBuilder<T> {
    pub fn edge(&mut self, u: usize, v: usize, l: T) {
        self.nodes.push(((v, l), self.heads[u]));
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
        let mut edges_i = self.nodes.len();
        let mut edges = Vec::<(usize, T)>::with_capacity(edges_i);
        let edges_p = edges.as_mut_ptr();
        *self.heads.last_mut().unwrap() = edges_i;
        unsafe {
            for h in self.heads.iter_mut().rev().skip(1) {
                let mut nodes_i = *h;
                while let Some((x, next)) = self.nodes.get(nodes_i) {
                    edges_i -= 1;
                    edges_p.add(edges_i).copy_from_nonoverlapping(x, 1);
                    nodes_i = *next;
                }
                *h = edges_i;
            }
            self.nodes.set_len(0);
            edges.set_len(edges.capacity());
        }
        LabeledGraph {
            heads: self.heads.into(),
            edges: edges.into(),
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