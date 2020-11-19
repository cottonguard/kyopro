pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}
pub struct LazySegTree<S, F, Map> {
    ss: Box<[S]>,
    fs: Box<[F]>,
    map: Map,
}
impl<S: Monoid, F: Monoid, Map: Fn(&F, &S) -> S> LazySegTree<S, F, Map> {
    pub fn new(n: usize, map: Map) -> Self {
        use std::iter::repeat_with;
        let len = 2 * n.next_power_of_two();
        Self {
            ss: repeat_with(S::id).take(len).collect(),
            fs: repeat_with(F::id).take(len).collect(),
            map,
        }
    }
    fn len(&self) -> usize {
        self.ss.len() / 2
    }
    fn propagate(&mut self, i: usize) {
        let h = 8 * std::mem::size_of::<usize>() as u32 - i.leading_zeros();
        for k in (1..h).rev() {
            let p = i >> k;
            let l = 2 * p;
            let r = 2 * p + 1;
            self.ss[l] = (self.map)(&self.fs[p], &self.ss[l]);
            self.ss[r] = (self.map)(&self.fs[p], &self.ss[r]);
            self.fs[l] = self.fs[p].op(&self.fs[l]);
            self.fs[r] = self.fs[p].op(&self.fs[r]);
            self.fs[p] = F::id();
        }
    }
    pub fn prod(&mut self, l: usize, r: usize) -> S {
        assert!(l <= r);
        assert!(r <= self.len());
        let mut l = l + self.len();
        let mut r = r + self.len();
        self.propagate(l >> l.trailing_zeros());
        self.propagate((r >> r.trailing_zeros()) - 1);
        let mut lv = S::id();
        let mut rv = S::id();
        while l < r {
            if l % 2 == 1 {
                lv = lv.op(&self.ss[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                rv = rv.op(&self.ss[r]);
            }
            l /= 2;
            r /= 2;
        }
        lv.op(&rv)
    }
    pub fn set(&mut self, i: usize, v: S) {
        let mut i = i + self.len();
        self.propagate(i);
        self.ss[i] = v;
        while i > 1 {
            i /= 2;
            self.ss[i] = self.ss[2 * i].op(&self.ss[2 * i + 1]);
        }
    }
    pub fn apply(&mut self, l: usize, r: usize, f: &F) {
        assert!(l <= r);
        assert!(r <= self.len());
        let mut li = l + self.len();
        let mut ri = r + self.len();
        let ln = li >> li.trailing_zeros();
        let rn = ri >> ri.trailing_zeros();
        self.propagate(ln);
        self.propagate(rn - 1);
        while li < ri {
            if li % 2 == 1 {
                self.fs[li] = f.op(&self.fs[li]);
                self.ss[li] = (self.map)(f, &self.ss[li]);
                li += 1;
            }
            if ri % 2 == 1 {
                ri -= 1;
                self.fs[ri] = f.op(&self.fs[ri]);
                self.ss[ri] = (self.map)(f, &self.ss[ri]);
            }
            li /= 2;
            ri /= 2;
        }
        let mut l = (l + self.len()) / 2;
        let mut r = (r + self.len() - 1) / 2;
        while l > 0 {
            if l < ln {
                self.ss[l] = self.ss[2 * l].op(&self.ss[2 * l + 1]);
            }
            if
            /*l != r && */
            r < rn - 1 {
                self.ss[r] = self.ss[2 * r].op(&self.ss[2 * r + 1]);
            }
            l /= 2;
            r /= 2;
        }
    }
}

impl<S: Monoid + Clone, F: Monoid, Map: Fn(&F, &S) -> S> LazySegTree<S, F, Map> {
    pub fn from_slice(a: &[S], map: Map) -> Self {
        use std::iter::repeat_with;
        let n = a.len();
        let len = 2 * n.next_power_of_two();
        let mut ss: Vec<_> = repeat_with(S::id).take(n).collect();
        ss.extend_from_slice(a);
        for i in (1..n).rev() {
            ss[i] = ss[2 * i].op(&ss[2 * i + 1]);
        }
        Self {
            ss: ss.into(),
            fs: repeat_with(F::id).take(len).collect(),
            map,
        }
    }
}
