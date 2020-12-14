pub fn zeta_superset<T, F: Fn(&T, &T) -> T>(a: &mut [T], k: usize, f: F) {
    assert!(a.len() >= 1 << k);
    for i in 0..k {
        let mut s = 0;
        while s < 1 << k {
            a[s] = f(&a[s], &a[s | 1 << i]);
            s += 1;
            if s & 1 << i != 0 {
                s += 1 << i;
            }
        }
    }
}
