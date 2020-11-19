pub fn berlekamp_massey(a: &[i64], m: i64) -> Vec<i64> {
    let mut c = vec![1];
    let mut pc = vec![1];
    let mut tc = Vec::new();
    let mut sh = 1;
    let mut pd = 1;
    for i in 0..a.len() {
        let mut d = a[i];
        for j in 1..c.len() {
            d = (d + c[j] * a[i - j]).rem_euclid(m);
        }
        if d == 0 {
            sh += 1;
        } else {
            let small = 2 * (c.len() - 1) <= a.len();
            if small {
                tc.clear();
                tc.extend_from_slice(&c);
                c.resize(i + 1 - (c.len() - 1) + 1, 0);
            }
            let e = (d * mod_inv(pd, m)).rem_euclid(m);
            for j in 0..pc.len() {
                c[sh + j] = (c[sh + j] - e * pc[j]).rem_euclid(m);
            }
            sh += 1;
            if small {
                std::mem::swap(&mut pc, &mut tc);
                pd = d;
                sh = 1;
            }
        }
    }
    c
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b != 0 {
        let (g, y, x) = extgcd(b, a.rem_euclid(b));
        (g, x, y - a / b * x)
    } else {
        (a, 1, 0)
    }
}
pub fn mod_inv(x: i64, m: i64) -> i64 {
    extgcd(x, m).1
}
