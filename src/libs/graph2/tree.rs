pub trait Tree<'a, T: 'a>: Graph<'a, T> {
    fn subtree_vertex_count(&'a self, root: usize) -> Vec<usize> {
        let mut res = vec![1; self.len()];
        let mut stk = vec![root];
        let mut par = vec![!0; self.len()];
        while let Some(u) = stk.pop() {
            if u as isize >= 0 {
                stk.push(!u);
                let p = par[u];
                stk.extend(self.adj_unlabeled(u).filter(|v| *v != p));
                for v in self.adj_unlabeled(u).filter(|v| *v != p) {
                    par[v] = u;
                }
            } else {
                let u = !u;
                let s = res[u];
                if let Some(r) = res.get_mut(par[u]) {
                    *r += s;
                }
            }
        }
        res
    }
    fn diameter(&'a self, v: usize) -> Vec<usize> {
        let dv = self.dist_bfs(v);
        let s = dv.iter().enumerate().max_by_key(|(_, d)| **d as isize).unwrap().0;
        let ds = self.dist_bfs(s);
        let t = ds.iter().enumerate().max_by_key(|(_, d)| **d as isize).unwrap().0;
        let mut path = Vec::with_capacity(ds[t] + 1);
        let mut u = t;
        while u != s {
            path.push(u);
            for v in self.adj_unlabeled(u) {
                if ds[u] == ds[v] + 1 {
                    u = v;
                    break;
                }
            }
        }
        path.push(s);
        path
    }
}
impl<'a, T: 'a, G: Graph<'a, T>> Tree<'a, T> for G {}