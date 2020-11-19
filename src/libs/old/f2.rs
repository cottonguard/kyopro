fn elimination_f2(a: &mut [u64]) -> usize {
    let n = a.len();
    let mut k = 0;
    for i in 0..n {
        if let Some(j) = (k..n).find(|&j| a[j] >> i & 1 != 0) {
            a.swap(k, j);
            for l in (0..n).filter(|&l| l != k) {
                a[l] ^= (a[l] >> i & 1) * a[k];
            }
            k += 1;
        }
    }
    k
}