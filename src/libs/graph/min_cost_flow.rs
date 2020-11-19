pub struct MinCostFlow {
    g: Vec<Vec<Edge>>,
}
#[derive(Clone, Copy)]
struct Edge {
    v: usize,
    rev: usize,
    cap: i32,
    cost: i64,
}
impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![Vec::new(); n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i32, cost: i64) {
        let rev_u = self.g[v].len();
        let rev_v = self.g[u].len();
        self.g[u].push(Edge {
            v,
            rev: rev_u,
            cap,
            cost,
        });
        self.g[v].push(Edge {
            v: u,
            rev: rev_v,
            cap: 0,
            cost: -cost,
        })
    }
    pub fn run(&mut self, s: usize, t: usize, limit: i32) -> (i32, i64) {
        use std::cmp::Reverse;
        let n = self.g.len();
        let mut flow = 0;
        let mut cost = 0;
        let mut p = vec![0; n];
        let mut dist = vec![0; n];
        let mut que = std::collections::BinaryHeap::new();
        let mut prev = vec![(0, 0); n];
        while flow < limit {
            dist.clear();
            dist.resize(n, i64::max_value());
            dist[s] = 0;
            que.push((Reverse(0), s));
            prev[t].0 = !0;
            while let Some((Reverse(d), u)) = que.pop() {
                if dist[u] < d {
                    continue;
                }
                for (i, e) in self.g[u].iter().enumerate() {
                    let dd = d + e.cost - p[e.v] + p[u];
                    if e.cap > 0 && dd < dist[e.v] {
                        dist[e.v] = dd;
                        que.push((Reverse(dd), e.v));
                        prev[e.v] = (u, i);
                    }
                }
            }
            if prev[t].0 == !0 {
                break;
            }
            for u in 0..n {
                if dist[u] != i64::max_value() {
                    p[u] += dist[u];
                }
            }
            let mut v = t;
            let mut add = limit - flow;
            while v != s {
                let (u, i) = prev[v];
                add = add.min(self.g[u][i].cap);
                v = u;
            }
            flow += add;
            let mut v = t;
            while v != s {
                let (u, i) = prev[v];
                let e = &mut self.g[u][i];
                cost += e.cost * add as i64;
                e.cap -= add;
                let rev = e.rev;
                self.g[v][rev].cap += add;
                v = u;
            }
        }
        (flow, cost)
    }
}
