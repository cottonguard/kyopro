#[derive(Debug)]
pub struct Hld {
    head: Vec<usize>,
    par: Vec<usize>,
    tab: Vec<usize>,
}
pub type Graph = [Vec<usize>];
impl Hld {
    pub fn new(g: &Graph, root: usize) -> Self {
        let mut heavy = vec![!0; g.len()];
        Self::dfs_heavy(g, &mut heavy, &mut vec![1; g.len()], root, !0);
        let mut hld = Hld {
            head: Vec::with_capacity(g.len()),
            par: Vec::with_capacity(g.len()),
            tab: vec![0; g.len()],
        };
        hld.dfs_build(g, &heavy, root, !0, root);
        hld
    }
    fn dfs_heavy(g: &Graph, heavy: &mut [usize], size: &mut [usize], u: usize, p: usize) {
        let mut max = 0;
        for &v in g[u].iter().filter(|&&v| v != p) {
            Self::dfs_heavy(g, heavy, size, v, u);
            if size[v] > max {
                max = size[v];
                heavy[u] = v;
            }
            size[u] += size[v];
        }
    }
    fn dfs_build(&mut self, g: &Graph, heavy: &[usize], u: usize, p: usize, h: usize) {
        self.tab[u] = self.head.len();
        self.head.push(h);
        self.par.push(p);
        if heavy[u] == !0 {
            return;
        }
        self.dfs_build(g, heavy, heavy[u], u, h);
        for &v in g[u].iter().filter(|&&v| v != p && v != heavy[u]) {
            self.dfs_build(g, heavy, v, u, v);
        }
    }
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.tab[u] > self.tab[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[self.tab[u]] == self.head[self.tab[v]] {
                return u;
            }
            v = self.par[self.tab[self.head[self.tab[v]]]];
        }
    }
}
