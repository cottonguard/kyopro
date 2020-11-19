pub struct UnionFind {
    p: Vec<isize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }
    pub fn root(&self, mut u: usize) -> usize {
        while self.p[u] >= 0 {
            u = self.p[u] as usize;
        }
        u
    }
    pub fn size(&self, u: usize) -> usize {
        (-self.p[self.root(u)]) as usize
    }
    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.root(u);
        let mut v = self.root(v);
        if u == v {
            return false;
        }
        if self.p[u] > self.p[v] {
            swap(&mut u, &mut v);
        }
        self.p[u] += self.p[v];
        self.p[v] = u as isize;
        true
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}