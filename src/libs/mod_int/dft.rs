use super::*;

pub trait PrimitiveRoot: Modulo {
    fn primitive_root() -> i32;
}
impl PrimitiveRoot for Mod998244353 {
    fn primitive_root() -> i32 {
        3
    }
}
pub fn dft<M: PrimitiveRoot>(a: &mut [ModInt<M>], inv: bool) {
    assert!(a.len().is_power_of_two());
    let n = a.len();
    let shift = n.leading_zeros() + 1;
    for i in 0..n {
        let j = i.reverse_bits().wrapping_shr(shift);
        if i < j {
            a.swap(i, j);
        }
    }
    let pr = ModInt::new(M::primitive_root());
    let mut w = Vec::with_capacity(n / 2);
    w.push(ModInt::new(1));
    for m in (1..).map(|i| 1 << i).take_while(|m| *m <= n) {
        let neg1 = M::modulo() as u64 - 1;
        let s = neg1 / m as u64;
        let w1 = if inv { pr.pow(neg1 - s) } else { pr.pow(s) };
        w.resize(m / 2, ModInt::new(0));
        for i in (0..m / 4).rev() {
            w[2 * i] = w[i];
            w[2 * i + 1] = w1 * w[i];
        }
        for i in (0..n).step_by(m) {
            for j in 0..m / 2 {
                let t = w[j] * a[i + j + m / 2];
                a[i + j + m / 2] = a[i + j] - t;
                a[i + j] += t;
            }
        }
    }
    if inv {
        let d = ModInt::new(n as i32).inv();
        for a in a {
            *a *= d;
        }
    }
}
pub fn convolution<M: PrimitiveRoot, V: Into<Vec<ModInt<M>>>>(a: V, b: V) -> Vec<ModInt<M>> {
    let mut a = a.into();
    let mut b = b.into();
    let deg = a.len() + b.len() - 1;
    let n = deg.next_power_of_two();
    a.resize(n, ModInt::new(0));
    b.resize(n, ModInt::new(0));
    dft(&mut a, false);
    dft(&mut b, false);
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a *= *b;
    }
    dft(&mut a, true);
    a.truncate(deg);
    a
}
