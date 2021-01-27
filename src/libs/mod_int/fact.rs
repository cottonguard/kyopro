use super::*;

pub struct Fact<M> {
    f: Vec<ModInt<M>>,
    finv: Vec<ModInt<M>>,
}
impl<M: Modulo> Fact<M> {
    pub fn new(n: usize) -> Self {
        let mut f = vec![ModInt::new(0); n + 1];
        f[0] = ModInt::new(1);
        for i in 1..=n {
            f[i] = ModInt::new(i as u32) * f[i - 1];
        }
        let mut finv = vec![ModInt::new(0); n + 1];
        finv[n] = f[n].inv();
        for i in (1..=n).rev() {
            finv[i - 1] = finv[i] * ModInt::new(i as u32);
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
