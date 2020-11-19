pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut z = vec![0; s.len()];
    z[0] = s.len();
    let mut i = 1;
    let mut p = 0;
    while i < s.len() {
        while s.get(p) == s.get(i + p) {
            p += 1;
        }
        z[i] = p;
        let mut k = 1;
        while k + z[k] < p {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        p = p.saturating_sub(k);
    }
    z
}
