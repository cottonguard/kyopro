pub fn two_edge_connected_components(g: &[Vec<usize>]) -> Vec<usize> {
    let ll = LowLink::new(g);
    let mut cc = vec![!0; g.len()];
    let mut col = 0;
    for u in 0..g.len() {
        if cc[u] == !0 {
            tecc_dfs(&ll, u, &mut cc, col);
            col += 1;
        }
    }
    cc
}
fn tecc_dfs(ll: &LowLink, u: usize, cc: &mut [usize], col: usize) {
    cc[u] = col;
    // detect multiedges
    let mut appeared = std::collections::BTreeSet::new();
    for &v in &ll.graph()[u] {
        if cc[v] == !0 {
            if !ll.is_bridge(u, v) || appeared.contains(&v) {
                tecc_dfs(ll, v, cc, col);
            } else {
                appeared.insert(v);
            }
        }
    }
}

#[derive(Debug)]
pub struct LowLink<'a> {
    g: &'a [Vec<usize>],
    ord: Vec<usize>,
    low: Vec<usize>,
}
impl<'a> LowLink<'a> {
    pub fn new(g: &'a [Vec<usize>]) -> Self {
        let mut ll = Self {
            g,
            ord: vec![!0; g.len()],
            low: vec![!0; g.len()],
        };
        let mut id = 0;
        for u in 0..g.len() {
            if ll.ord[u] == !0 {
                ll.dfs(u, !0, &mut id);
            }
        }
        ll
    }
    fn dfs(&mut self, u: usize, p: usize, id: &mut usize) {
        self.ord[u] = *id;
        self.low[u] = *id;
        *id += 1;
        for &v in self.g[u].iter().filter(|&&v| v != p) {
            if self.ord[v] == !0 {
                self.dfs(v, u, id);
            }
            self.low[u] = self.low[u].min(self.low[v]);
        }
    }
    pub fn graph(&self) -> &[Vec<usize>] {
        &self.g
    }
    pub fn is_bridge(&self, mut u: usize, mut v: usize) -> bool {
        if self.ord[u] > self.ord[v] {
            std::mem::swap(&mut u, &mut v)
        }
        self.ord[u] < self.low[v]
    }
}
