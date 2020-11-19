pub struct Dinic {
    g: Vec<Vec<(usize, bool)>>,
    es: Vec<(usize, usize, i64)>,
}
impl Dinic {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![Vec::new(); n],
            es: Vec::new(),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        self.g[u].push((self.es.len(), true));
        self.g[v].push((self.es.len(), false));
        self.es.push((u, v, cap));
    }
    fn edge(&self, flow: &[i64], i: usize, fw: bool) -> (usize, i64) {
        let (u, v, c) = self.es[i];
        if fw {
            (v, c - flow[i])
        } else {
            (u, flow[i])
        }
    }
    pub fn run(&self, s: usize, t: usize) -> (i64, Vec<i64>) {
        let mut flow = vec![0; self.es.len()];
        let mut sum = 0;
        loop {
            let mut dist = vec![!0; self.g.len()];
            dist[s] = 0;
            let mut que = std::collections::VecDeque::new();
            que.push_back(s);
            'bfs: while let Some(u) = que.pop_front() {
                for (v, r) in self.g[u].iter().map(|&(i, fw)| self.edge(&flow, i, fw)) {
                    if dist[v] == !0 && r > 0 {
                        dist[v] = dist[u] + 1;
                        if v == t {
                            break 'bfs;
                        }
                        que.push_back(v);
                    }
                }
            }
            let add = self.dfs(&mut flow, &mut dist, s, t, i64::max_value());
            if add == 0 {
                break;
            }
            sum += add;
        }
        (sum, flow)
    }
    fn dfs(&self, flow: &mut [i64], dist: &mut [usize], u: usize, t: usize, cap: i64) -> i64 {
        if u == t {
            return cap;
        }
        let mut add = 0;
        for &(i, fw) in &self.g[u] {
            let (v, r) = self.edge(&flow, i, fw);
            if dist[v] == dist[u] + 1 && r > 0 {
                let a = self.dfs(flow, dist, v, t, (cap - add).min(r));
                if a > 0 {
                    if fw {
                        flow[i] += a;
                    } else {
                        flow[i] -= a;
                    }
                    add += a;
                } else {
                    dist[v] = 0;
                }
            }
        }
        add
    }
}
