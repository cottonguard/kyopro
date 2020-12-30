pub fn zeta_superset<T, F: Fn(&T, &T) -> T>(a: &mut [T], k: usize, f: F) {
    assert!(a.len() >= 1 << k);
    for i in 0..k {
        let mut s = 0;
        while s < 1 << k {
            a[s] = f(&a[s], &a[s | 1 << i]);
            s += 1;
            s += s & 1 << i;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn zeta_superset() {
        let a = vec![3i32, 1, 4, 1, 5, 9, 2, 6];
        let mut zeta = a.clone();
        super::zeta_superset(&mut zeta, 3, |x, y| x + y);
        let mut naive = vec![0; 8];
        for s in 0..1 << 3 {
            for t in 0..1 << 3 {
                if s | t == t {
                    naive[s] += a[t];
                }
            }
        }
        assert_eq!(zeta, naive);
    }
}
