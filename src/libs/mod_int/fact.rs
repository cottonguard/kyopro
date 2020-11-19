use super::*;

pub struct Fact<M>(Vec<ModInt<M>>);
impl<M: Modulo> Fact<M> {
    pub fn new(n: usize) -> Self {
        let mut f = vec![ModInt::new(1); n + 1];
        for i in 2..=n {
            f[i] = ModInt::new(i as i32) * f[i - 1];
        }
        Self(f)
    }
    pub fn fact(&self, x: usize) -> ModInt<M> {
        self.0[x]
    }
    pub fn binom(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) / (self.fact(n - k) * self.fact(k))
        } else {
            ModInt::new(0)
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) / self.fact(k)
        } else {
            ModInt::new(0)
        }
    }
}
