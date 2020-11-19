pub const MOD: i64 = 998244353;
pub const N: usize = 500;
#[derive(Clone)]
pub struct SquareMatrix(Box<[[i64; N]]>);
impl SquareMatrix {
    pub fn zeros() -> Self {
        Self(vec![[0; N]; N].into())
    }
    pub fn id() -> Self {
        let mut a = Self::zeros();
        for i in 0..N {
            a[i][i] = 1;
        }
        a
    }
    pub fn add(&self, rhs: &Self) -> Self {
        let mut a = Self::zeros();
        for i in 0..N {
            for j in 0..N {
                a[i][j] = (self[i][j] + rhs[i][j]).rem_euclid(MOD);
            }
        }
        a
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut a = Self::zeros();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    a[i][j] = (self[i][k] + rhs[k][j]).rem_euclid(MOD);
                }
            }
        }
        a
    }
    pub fn pow(&self, mut k: i64) -> Self {
        let mut a = self.clone();
        let mut b = Self::id();
        while k > 0 {
            if k & 1 == 1 {
                b = b.mul(&a);
            }
            a = a.mul(&a);
            k >>= 1;
        }
        b
    }
    // wip https://en.wikipedia.org/wiki/LU_decomposition
    pub fn lup(&mut self) -> Option<(Vec<usize>, i64)> {
        let mut p: Vec<_> = (0..N).collect();
        let mut swap_count = 0;
        let mut d_inv = vec![0; N];
        for k in 0..N {
            if self[k][k] == 0 {
                for i in k + 1..N {
                    if self[i][k] != 0 {
                        self.0.swap(i, k);
                        p.swap(i, k);
                        swap_count += 1;
                    }
                }
                if self[k][k] == 0 {
                    return None;
                }
            }
            d_inv[k] = mod_inv(self[k][k], MOD);
            for i in k + 1..N {
                self[i][k] = (self[i][k] * d_inv[k]).rem_euclid(MOD);
                for j in k + 1..N {
                    self[i][j] = (self[i][j] - self[i][k] * self[k][j]).rem_euclid(MOD);
                }
            }
        }
        Some((p, swap_count))
    }
    pub fn det(&mut self) -> i64 {
        if let Some((_, c)) = self.lup() {
            let mut det = if c & 1 == 0 { 1 } else { -1 };
            for i in 0..N {
                det = (det * self[i][i]).rem_euclid(MOD);
            }
            det
        } else {
            0
        }
    }
    pub fn inv(mut self) -> Option<Self> {
        self.lup().map(|(p, _)| {
            let mut inv = Self::zeros();
            for i in 0..N {
                inv[i][p[i]] = 1;
            }
            for k in 0..N {
                for i in k + 1..N {
                    for j in 0..N {
                        inv[i][j] = (inv[i][j] - self[i][k] * inv[k][j]).rem_euclid(MOD);
                    }
                }
            }
            for k in (0..N).rev() {
                let d = mod_inv(self[k][k], MOD);
                for i in 0..k {
                    let c = (d * self[i][k]).rem_euclid(MOD);
                    for j in 0..N {
                        inv[i][j] = (inv[i][j] - c * inv[k][j]).rem_euclid(MOD);
                    }
                }
                for j in 0..N {
                    inv[k][j] = (d * inv[k][j]).rem_euclid(MOD);
                }
            }
            inv
        })
    }
}
use std::ops::*;
impl<I: std::slice::SliceIndex<[[i64; N]]>> Index<I> for SquareMatrix {
    type Output = I::Output;
    fn index(&self, i: I) -> &Self::Output {
        &self.0[i]
    }
}
impl<I: std::slice::SliceIndex<[[i64; N]]>> IndexMut<I> for SquareMatrix {
    fn index_mut(&mut self, i: I) -> &mut Self::Output {
        &mut self.0[i]
    }
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