// wip!!!!!!!!
use super::{dft::dft, Mod998244353, ModInt};
use std::ops;
type Mint = ModInt<Mod998244353>;
#[derive(Clone)]
pub struct Poly(Vec<Mint>);
impl Poly {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn inv(self, n: usize) -> Self {
        // by Newton's method
        // X_n+1 = X_n - (1 / X_n - A) / (-1 / (X_n)^2)
        //       = X_n - (-X_n + A * (X_n)^2)
        //       = 2X_n - A * (X_n)^2
        let cap = n.next_power_of_two();
        let mut x = Self(Vec::with_capacity(cap));
        x.0.push(self[0].inv());
        let mut x_tmp = Self(Vec::with_capacity(cap));
        let mut b = Self(Vec::with_capacity(2 * cap));
        let mut len = 1;
        while len < n {
            len *= 2;
            x_tmp.0.clear();
            x_tmp.0.extend_from_slice(&x);
            b.0.clear();
            b.0.extend_from_slice(&self[..self.len().min(len)]);
            b.0.resize(2 * len, ModInt::new(0));
            x.0.resize(2 * len, ModInt::new(0));
            dft(&mut b, false);
            dft(&mut x, false);
            for (b, &x) in b.iter_mut().zip(x.iter()) {
                *b *= x * x;
            }
            dft(&mut b, true);
            std::mem::swap(&mut x, &mut x_tmp);
            x.0.resize(len, ModInt::new(0));
            for (x, &b) in x.iter_mut().zip(b.iter()).skip(len / 2) {
                *x = *x + *x - b;
            }
        }
        x.truncate(n);
        x
    }
    pub fn truncate(&mut self, n: usize) {
        self.0.truncate(n);
    }
}
impl ops::Deref for Poly {
    type Target = [Mint];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for Poly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ops::Neg for Poly {
    type Output = Poly;
    fn neg(mut self) -> Poly {
        for a in &mut *self {
            *a = -*a;
        }
        self
    }
}
