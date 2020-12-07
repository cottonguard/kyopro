use super::*;

pub fn mod_inv_table<M: Modulo>(n: usize) -> Vec<ModInt<M>> {
    let mut inv = vec![ModInt::new(0); n + 1];
    inv[1] = ModInt::new(1);
    for x in 2..=n {
        let div = M::modulo() as usize / x;
        let rem = M::modulo() as usize % x;
        inv[x] = inv[rem] * -ModInt::new(div as u32);
    }
    inv
}
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
        let inv = mod_inv_table(n);
        let mut finv = vec![ModInt::new(0); n + 1];
        finv[0] = ModInt::new(1);
        finv[1] = ModInt::new(1);
        for i in 2..=n {
            finv[i] = inv[i] * finv[i - 1];
        }
        Self {
            f,
            finv
        }
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

