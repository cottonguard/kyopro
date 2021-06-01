pub fn strongly_connected_components(g: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = g.len();
    Scc {
        g,
        stk: vec![],
        ord: vec![0; n],
        low: vec![0; n],
        idx: 1,
        comp_id: 0,
    }
    .run()
}
struct Scc<'a> {
    g: &'a [Vec<usize>],
    stk: Vec<usize>,
    ord: Vec<usize>,
    low: Vec<usize>,
    idx: usize,
    comp_id: usize,
}
impl<'a> Scc<'a> {
    fn run(mut self) -> (usize, Vec<usize>) {
        for r in 0..self.g.len() {
            if self.ord[r] == 0 {
                self.rec(r);
            }
        }
        for c in &mut self.ord {
            *c = (self.comp_id as isize + *c as isize) as usize;
        }
        (self.comp_id, self.ord)
    }
    fn rec(&mut self, u: usize) {
        self.ord[u] = self.idx;
        self.low[u] = self.idx;
        self.idx += 1;
        self.stk.push(u);
        for &v in &self.g[u] {
            if self.ord[v] == 0 {
                self.rec(v);
                self.low[u] = self.low[u].min(self.low[v]);
            } else {
                self.low[u] = self.low[u].min(self.ord[v]);
            }
        }
        if self.ord[u] == self.low[u] {
            let i = self.stk.iter().rposition(|v| *v == u).unwrap();
            self.comp_id += 1;
            for v in self.stk.drain(i..) {
                self.ord[v] = -(self.comp_id as isize) as usize;
            }
        }
    }
}
