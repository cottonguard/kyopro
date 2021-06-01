use super::*;
use crate::random::*;
use fenwick_tree::FenwickTree;
use lazy_seg_tree::{LazySegTree, Map};
use sparse_table::SparseTable;
use wavelet_matrix::WaveletMatrix;

#[test]
fn sparse_table() {
    const N: usize = 19;
    let mut rand = Pcg::seed_from_u64(11922960);
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
    let mut rand = Pcg::seed_from_u64(11922960);
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
fn seg_tree_max() {
    impl Monoid for u32 {
        fn id() -> u32 {
            0
        }
        fn op(&self, y: &u32) -> u32 {
            *self.max(y)
        }
    }
    const N: usize = 20;
    let mut rand = Pcg::seed_from_u64(8181);
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

#[test]
fn lazy_seg_tree_range_add_range_max() {
    struct Add(u32);
    impl Monoid for Add {
        fn id() -> Self {
            Add(0)
        }
        fn op(&self, other: &Self) -> Self {
            Add(self.0 + other.0)
        }
    }
    struct Max(u32);
    impl Monoid for Max {
        fn id() -> Self {
            Max(0)
        }
        fn op(&self, other: &Self) -> Self {
            Max(self.0.max(other.0))
        }
    }
    impl Map<Max> for Add {
        fn map(&self, x: Max) -> Max {
            Max(self.0 + x.0)
        }
    }
    const N: usize = 20;
    let mut rand = Pcg::seed_from_u64(1818);
    let mut lst = LazySegTree::<Max, Add>::new(N);
    let mut a = vec![0; N];
    for i in 0..30 {
        if i % 3 == 0 {
            let l = rand.next_u32() as usize % N;
            let r = l + rand.next_u32() as usize % (N - l) + 1;
            let stmax = lst.prod(l, r);
            let amax = *a[l..r].iter().max().unwrap();
            assert_eq!(stmax.0, amax);
        } else {
            let l = rand.next_u32() as usize % N;
            let r = l + rand.next_u32() as usize % (N - l) + 1;
            let x = rand.next_u32() / N as u32;
            lst.apply(l, r, &Add(x));
            for a in &mut a[l..r] {
                *a += x;
            }
        }
    }
}

#[test]
fn wavelet_matrix_rank() {
    use std::iter;
    const N: usize = 1000;
    const K: usize = 50;
    let mut rand = Pcg::seed_from_u64(1352);
    let values: Vec<_> = iter::repeat_with(|| rand.next_u32()).take(K).collect();
    let mut cnt = [0; 100];
    let a: Vec<_> = iter::repeat_with(|| {
        let i = rand.next_u32() as usize % K;
        cnt[i] += 1;
        values[i]
    })
    .take(N)
    .collect();
    let wm = WaveletMatrix::new(a);
    for i in 0..K {
        assert_eq!(wm.rank(0, N, values[i]), cnt[i]);
    }
}
