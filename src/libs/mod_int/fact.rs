use super::*;

pub struct Fact<M> {
    f: Vec<ModInt<M>>,
    finv: Vec<ModInt<M>>,
}
impl<M: Modulo> Fact<M> {
    pub fn new(n: usize) -> Self {
        let mut f = vec![ModInt::new(0); n + 1];
        f[0] = ModInt::new(1);
        f[1] = ModInt::new(1);
        for i in 2..=n {
            f[i] = ModInt::new(i as u32) * f[i - 1];
        }
        let mut finv = vec![ModInt::new(0); n + 1];
        finv[n] = f[n].inv();
        for i in (0..n).rev() {
            finv[i] = finv[i + 1] * ModInt::new(i as u32 + 1);
        }
        Self { f, finv }
    }
    pub fn fact(&self, x: usize) -> ModInt<M> {
        self.f[x]
    }
    pub fn fact_inv(&self, x: usize) -> ModInt<M> {
        self.finv[x]
    }
    pub fn binom(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k) * self.fact_inv(k)
        } else {
            ModInt::new(0)
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k)
        } else {
            ModInt::new(0)
        }
    }
}
