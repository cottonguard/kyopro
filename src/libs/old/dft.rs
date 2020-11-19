const P: i64 = 998244353; // 2^23 * 7 * 17 + 1
const PR: i64 = 3;

pub fn dft(a: &mut [i64], inv: bool) {
    // Cooley–Tukey FFT
    debug_assert!(a.len().is_power_of_two());
    let n = a.len();
    for i in 0..n {
        let j = i.reverse_bits().wrapping_shr(n.leading_zeros() + 1);
        if i < j {
            a.swap(i, j);
        }
    }
    let mut w = Vec::with_capacity(n / 2);
    for k in (1..).map(|d| 1 << d).take_while(|&k| k <= n) {
        let r = if inv {
            mod_pow(PR, (P - 1) / k as i64, P)
        } else {
            mod_pow(PR, P - 1 - (P - 1) / k as i64, P)
        };
        w.clear();
        w.extend(std::iter::successors(Some(1), |&w| Some(w * r % P)).take(k / 2));
        for ofs in (0..n).step_by(k) {
            for i in 0..k / 2 {
                let u = ofs + i;
                let v = ofs + i + k / 2;
                let t = w[i] * a[v];
                a[v] = (a[u] - t).rem_euclid(P);
                a[u] = (a[u] + t) % P;
            }
        }
    }
    if inv {
        let d = mod_pow(n as i64, P - 2, P);
        for v in a {
            *v = d * *v % P;
        }
    }
}
pub fn convolution(a: &mut Vec<i64>, b: &mut Vec<i64>) {
    let n = a.len() + b.len() - 1;
    let m = n.next_power_of_two();
    a.resize(m, 0);
    b.resize(m, 0);
    dft(a, false);
    dft(b, false);
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x = *x * *y % P;
    }
    dft(a, true);
    a.truncate(n);
}
pub fn mod_pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut y = 1;
    while b > 0 {
        if b & 1 == 1 {
            y = y * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    y
}