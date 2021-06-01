use std::cmp::Ordering;
pub fn nth_element<T: Ord>(a: &mut [T], n: usize) {
    nth_element_by(a, n, |x, y| x.cmp(y));
}
pub fn nth_element_by_key<T, K: Ord, F: FnMut(&T) -> K>(a: &mut [T], n: usize, mut f: F) {
    nth_element_by(a, n, |x, y| f(x).cmp(&f(y)));
}
pub fn nth_element_by<T, F: FnMut(&T, &T) -> Ordering>(a: &mut [T], n: usize, mut f: F) {
    assert!(n < a.len());
    let mut l = 0;
    let mut r = a.len();
    while r - l >= 2 {
        let mut pis = [l, (l + r) / 2, r - 1];
        pis.sort_by(|i, j| f(&a[*i], &a[*j]));
        a.swap(pis[1], r - 1);
        let (b, t) = a.split_at_mut(r - 1);
        let pivot = &t[0];
        let mut i = l;
        let mut j = r - 2;
        while i <= j {
            while i <= j && f(&b[i], pivot) == Ordering::Less {
                i += 1;
            }
            while i < j && f(&b[j], pivot) == Ordering::Greater {
                j -= 1;
            }
            if i >= j {
                break;
            }
            b.swap(i, j);
            i += 1;
            j -= 1;
        }
        a.swap(i, r - 1);
        if i < n {
            l = i + 1;
        } else {
            r = i;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nth_element() {
        use crate::random::*;

        const N: usize = 50;
        const M: u32 = 20;
        const T: usize = 100;
        let mut rand = Pcg::seed_from_u64(1958);
        for _ in 0..T {
            let mut a: Vec<_> = (0..50).map(|_| rand.next_u32() % M).collect();
            let n = rand.next_u32() as usize % N;
            super::nth_element(&mut a, n);
            for i in 0..n {
                assert!(a[i] <= a[n]);
            }
            for i in n..N {
                assert!(a[i] >= a[n]);
            }
        }
    }
}
