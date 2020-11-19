use super::*;
use crate::random::Pcg;
use fenwick_tree::FenwickTree;
use segment_tree::{Monoid, SegmentTree};
use sparse_table::SparseTable;

#[test]
fn sparse_table() {
    const N: usize = 19;
    let mut rand = Pcg::new(11922960);
    let st: SparseTable<_> = (0..N).map(|_| rand.next_u32()).collect();
    for _ in 0..20 {
        let l = rand.next_u32() as usize % N;
        let r = l + rand.next_u32() as usize % (N - l) + 1;
        let min = st[l..r].iter().min().unwrap();
        assert_eq!(st.min(l, r), min);
    }
}

#[test]
fn fenwick_tree_add() {
    const N: usize = 20;
    let mut rand = Pcg::new(11922960);
    let mut ft = FenwickTree::new(N, || 0, |x, y| x + y);
    let mut a = vec![0; N];
    for i in 0..30 {
        if i % 3 == 0 {
            let r = rand.next_u32() as usize % (N + 1);
            let asum: u32 = a[..r].iter().sum();
            let ftsum = ft.sum(r);
            assert_eq!(asum, ftsum);
        } else if i == 20 {
            ft.reset();
            for v in &mut a {
                *v = 0;
            }
        } else {
            let i = rand.next_u32() as usize % N;
            let x = rand.next_u32() >> 5;
            ft.add(i, x);
            a[i] += x;
        }
    }
}

#[test]
fn dsu_with_data_drop() {
    use std::{cell::Cell, rc::Rc};
    struct S(i32, Rc<Cell<i32>>);
    impl Drop for S {
        fn drop(&mut self) {
            let x = self.1.get();
            self.1.set(x - self.0);
        }
    }
    let n = 23;
    let cnt = Rc::new(Cell::new(n * (n + 1) / 2));
    let mut dsu: dsu::DsuWithData<_> = (1..=n).map(|i| S(i, cnt.clone())).collect();
    for i in 0..n as usize / 3 {
        dsu.unite(i, i * i % n as usize, |s, t| {
            s.0 += t.0;
            std::mem::forget(t);
        });
    }
    drop(dsu);
    assert_eq!(cnt.get(), 0);
}

#[test]
fn segtree_max() {
    impl Monoid for u32 {
        fn id() -> u32 {
            0
        }
        fn op(&self, y: &u32) -> u32 {
            *self.max(y)
        }
    }
    const N: usize = 20;
    let mut rand = Pcg::new(8181);
    let mut a: Vec<_> = (0..N).map(|_| rand.next_u32()).collect();
    let mut st: SegmentTree<_> = a.iter().copied().collect();
    for i in 0..30 {
        if i % 3 == 0 {
            let l = rand.next_u32() as usize % N;
            let r = l + rand.next_u32() as usize % (N - l) + 1;
            let stmax = st.prod(l, r);
            let amax = *a[l..r].iter().max().unwrap();
            assert_eq!(stmax, amax);
        } else {
            let i = rand.next_u32() as usize % N;
            let x = rand.next_u32();
            if i % 3 == 1 {
                st.set(i, x);
                a[i] = x;
            } else {
                st.update(i, &x);
                a[i] = a[i].max(x);
            }
        }
    }
}
