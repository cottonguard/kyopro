pub fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let px = x.trailing_zeros();
    let py = y.trailing_zeros();
    let (mut x, mut y) = (x >> px, y >> py);
    while x != y {
        if x > y {
            swap(&mut x, &mut y);
        }
        y = y - x;
        y >>= y.trailing_zeros();
    }
    x << px.min(py)
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