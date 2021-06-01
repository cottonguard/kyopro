pub trait GraphExt<'a, T: 'a>: Graph<'a, T> {
    fn tsort(&'a self) -> Option<Vec<usize>> {
        let mut res = Vec::with_capacity(self.len());
        let mut state = vec![0u8; self.len()];
        let mut stk = vec![];
        for s in (0..self.len()).rev() {
            if state[s] != 0 {
                debug_assert_ne!(state[s], 1);
                continue;
            }
            state[s] = 1;
            stk.push((s, self.adj(s)));
            while let Some((u, adj)) = stk.last_mut() {
                if let Some((v, _)) = adj.next() {
                    match state[v] {
                        1 => return None,
                        2 => continue,
                        _ => {}
                    }
                    state[v] = 1;
                    stk.push((v, self.adj(v)));
                } else {
                    state[*u] = 2;
                    res.push(*u);
                    stk.pop();
                }
            }
        }
        res.reverse();
        Some(res)
    }
    fn djikstra(&'a self, s: usize, init: T) -> Vec<Option<T>>
    where
        T: Clone + Ord,
        for <'b> &'b T: std::ops::Add<&'b T, Output = T>
    {
        use std::cmp::Reverse;
        let mut res: Vec<_> = (0..self.len()).map(|_| None).collect();
        res[s] = Some(init.clone());
        let mut que: BinaryHeap<_> = Some((Reverse(init), s)).into_iter().collect();
        while let Some((Reverse(dsu), u)) = que.pop() {
            if res[u].as_ref().map(|d| d > &dsu).unwrap_or(false) {
                continue;
            }
            for (v, w) in self.adj(u) {
                let dsuv = &dsu + &w;
                if res[v].as_ref().map(|d| &dsuv < d).unwrap_or(true) {
                    res[v] = Some(dsuv.clone());
                    que.push((Reverse(dsuv), v));
                }
            }
        }
        res
    }
    fn dist_bfs(&'a self, s: usize) -> Vec<usize> {
        let mut que = VecDeque::new();
        que.push_back(s);
        let mut res = vec![!0; self.len()];
        res[s] = 0;
        while let Some(u) = que.pop_front() {
            let d = res[u];
            for (v, _) in self.adj(u) {
                if res[v] == !0 {
                    res[v] = d + 1;
                    que.push_back(v);
                }
            }
        }
        res
    }
}
impl<'a, T: 'a, G: Graph<'a, T>> GraphExt<'a, T> for G {}