pub fn primes(n: usize) -> Vec<usize> {
    // 1, 7, 11, 13, 17, 19, 23, 29
    const SKIP: [u8; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
    const XTOI: [u8; 15] = [0, 0, 0, 1, 0, 2, 3, 0, 4, 5, 0, 6, 0, 0, 7];
    let mut sieve = vec![0u8; n / 30 + 1];
    let mut ps = vec![2, 3, 5];
    if n <= 4 {
        ps.truncate([0, 0, 1, 2, 2][n]);
        return ps;
    }
    let mut x = 7;
    let mut i = 1;
    while x <= n {
        if sieve[i / 8] & 1 << i % 8 == 0 {
            ps.push(x);
            let mut j = i;
            let mut y = x * x;
            while y <= n {
                sieve[y / 30] |= 1 << XTOI[y / 2 % 15];
                y += x * SKIP[j % 8] as usize;
                j += 1;
            }
        }
        x += SKIP[i % 8] as usize;
        i += 1;
    }
    ps
}

pub fn prime_count(n: usize) -> usize {
    let sn = (n as f64).sqrt().round() as usize;
    let mut ps = Vec::new();
    ps.push(1);
    let mut sieve = vec![true; sn + 1];
    for i in 2..=sn {
        if sieve[i] {
            ps.push(i);
            for j in (i * i..=sn).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    let div: Vec<_> = (0..=sn).map(|i| n.checked_div(i).unwrap_or(0)).collect();
    let mut dp_large: Vec<_> = div.iter().map(|&i| i.saturating_sub(1)).collect();
    let mut dp_small: Vec<_> = std::iter::once(0).chain(0..sn).collect();
    for (i, p) in ps.into_iter().enumerate().skip(1) {
        let diff = |k, dp_large: &[usize], dp_small: &[usize]| if k > sn {
                dp_large[n / k]
            } else {
                dp_small[k]
            } + 1 - i;
        for j in (1..=sn).take_while(|&j| div[j] >= p * p) {
            dp_large[j] -= diff(div[j] / p, &dp_large, &dp_small);
        }
        for j in (p * p..=sn).rev() {
            dp_small[j] -= diff(j / p, &dp_large, &dp_small);
        }
    }
    dp_large[1]
}

pub fn mod_sqrt(n: i64, p: i64) -> Option<i64> {
    if p == 2 || n == 0 {
        Some(n)
    } else if p % 4 == 3 {
        let r = mod_pow(n, (p + 1) / 4, p);
        if r * r % p == n {
            Some(r)
        } else {
            None
        }
    } else if legendre_symbol(n, p) != 1 {
        None
    } else {
        tonelli_shanks(n, p)
    }
}
fn tonelli_shanks(n: i64, p: i64) -> Option<i64> {
    let mut z = 2;
    while legendre_symbol(z, p) != p - 1 {
        z = (z * z - 1).rem_euclid(p);
    }
    // p - 1 == q * (2 ** s)
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let mut m = s;
    let mut t = mod_pow(n, q, p);
    let mut c = mod_pow(z, q, p);
    let mut r = mod_pow(n, (q + 1) / 2, p);
    while m > 0 {
        let mut st = t;
        let mut i = 0;
        while st != 1 {
            st = st * st % p;
            i += 1;
        }
        if i == m {
            return None;
        }
        for _ in 0..m - i - 1 {
            c = c * c % p;
        }
        m = i;
        r = r * c % p;
        c = c * c % p;
        t = t * c % p;
    }
    Some(r)
}
fn legendre_symbol(n: i64, p: i64) -> i64 {
    mod_pow(n, (p - 1) / 2, p)
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

pub fn totient_table(n: usize) -> Vec<usize> {
    let mut t: Vec<usize> = (0..=n).collect();
    for i in 2..=n {
        if t[i] == i {
            for j in (i..=n).step_by(i) {
                t[j] -= t[j] / i;
            }
        }
    }
    t
}

pub fn totient_sum(n: i64, p: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let m = (n as f64).powf(2. / 3.).ceil() as i64;
    // small[i] == Phi(i)
    let small = totient_sum_table(m, p);
    // large[i] == Phi(n / i)
    let mut large = vec![0; (n / m) as usize + 1];
    for di in (1..=n / m).rev() {
        let i = n / di;
        let si = (i as f64).sqrt().round() as i64;
        let mut sum = 0;
        for j in 1..=si {
            sum = (sum + (i / j - i / (j + 1)) * small[j as usize]) % p;
        }
        for dj in 2.. {
            let j = i / dj;
            if j <= si {
                break;
            }
            sum += if j <= m {
                small[j as usize]
            } else {
                large[(n / j) as usize]
            };
        }
        let ri = i % p;
        large[di as usize] = (ri * (ri + 1) / 2 - sum).rem_euclid(p);
    }
    large[1]
}
fn totient_sum_table(n: i64, p: i64) -> Vec<i64> {
    let mut tt: Vec<_> = (0..=n).collect();
    for i in 2..=n as usize {
        if tt[i] == i as i64 {
            for j in (i..=n as usize).step_by(i) {
                tt[j] = tt[j] - tt[j] / i as i64;
            }
        }
        tt[i] = (tt[i] + tt[i - 1]) % p;
    }
    tt
}
