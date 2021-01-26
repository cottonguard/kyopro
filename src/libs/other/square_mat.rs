use std::{
    mem::{self, MaybeUninit, ManuallyDrop},
    ops::{Neg, Add, AddAssign, Sub, SubAssign, Deref, DerefMut, Mul, MulAssign},
};
pub const N: usize = 3;
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct SquareMat<T>([[T; N]; N]);
impl<T: Copy + From<i8>> SquareMat<T> {
    pub fn zero() -> Self {
        Self([[0i8.into(); N]; N])
    }
    pub fn id() -> Self {
        let mut a = Self::zero();
        for i in 0..N {
            a[i][i] = 1i8.into();
        }
        a
    }
}
impl<T> SquareMat<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub fn uninit() -> SquareMat<MaybeUninit<T>> {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
    pub fn transpose(mut self) -> Self {
        for i in 0..N {
            let (u, d) = self.split_at_mut(i);
            for j in 0..i {
                mem::swap(&mut u[j][i], &mut d[0][j]);
            }
        }
        self
    }
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> SquareMat<U> {
        unsafe {
            let mut a: SquareMat<ManuallyDrop<T>> = mem::transmute_copy(&self);
            let mut res = SquareMat::uninit();
            for i in 0..N {
                for j in 0..N {
                    res[i][j] = MaybeUninit::new(f(ManuallyDrop::take(&mut a[i][j])));
                }
            }
            res.assume_init()
        }
    }
}
impl<T> SquareMat<MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> SquareMat<T> {
        mem::transmute_copy(&self)
    }
}
impl<T: Copy + From<i8> + Add<Output = T> + Mul<Output = T>> SquareMat<T> {
    pub fn pow<U: Into<u64>>(mut self, n: U) -> Self {
        let mut n = n.into();
        let mut y = Self::id();
        while n > 0 {
            if n % 2 == 1 {
                y *= self;
            }
            self *= self;
            n /= 2;
        }
        y
    }
}
impl<T> Deref for SquareMat<T> {
    type Target = [[T; N]; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for SquareMat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Copy + Neg<Output = T>> Neg for SquareMat<T> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            for j in 0..N {
                self[i][j] = -self[i][j];
            }
        }
        self
    }
}
impl<T: Copy + Add<Output = T>> Add for SquareMat<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::uninit();
        for i in 0..N {
            for j in 0..N {
                res[i][j] = MaybeUninit::new(self[i][j] + rhs[i][j]);
            }
        }
        unsafe { res.assume_init() }
    }
}
impl<T: Copy + AddAssign> AddAssign for SquareMat<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..N {
                self[i][j] += rhs[i][j];
            }
        }
    }
}
impl<T: Copy + Sub<Output = T>> Sub for SquareMat<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::uninit();
        for i in 0..N {
            for j in 0..N {
                res[i][j] = MaybeUninit::new(self[i][j] - rhs[i][j]);
            }
        }
        unsafe { res.assume_init() }
    }
}
impl<T: Copy + SubAssign> SubAssign for SquareMat<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..N {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}
impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul for SquareMat<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Self::uninit();
        for i in 0..N {
            for j in 0..N {
                let mut x = self[i][0] * rhs[0][j];
                for k in 1..N {
                    x = x + self[i][k] * rhs[k][j];
                }
                res[i][j] = MaybeUninit::new(x);
            }
        }
        unsafe { res.assume_init() }
    }
}
impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul<[T; N]> for SquareMat<T> {
    type Output = [T; N];
    fn mul(self, rhs: [T; N]) -> Self::Output {
        let mut res: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let mut x = self[i][0] * rhs[0];
            for j in 0..N {
                x = x + self[i][j] * rhs[j];
            }
            res[i] = MaybeUninit::new(x);
        }
        unsafe { mem::transmute_copy(&res) }
    }
}
impl<T: Copy + Add<Output = T> + Mul<Output = T>> MulAssign for SquareMat<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<T> From<[[T; N]; N]> for SquareMat<T> {
    fn from(a: [[T; N]; N]) -> Self {
        Self(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow() {
        let (sin, cos) = (std::f64::consts::PI / 13.).sin_cos();
        let a: SquareMat<_> = [[cos, -sin, 0.], [sin, cos, 0.], [0., 0., 1.]].into();
        let b = a.pow(26u64);
        let id = SquareMat::<f64>::id();
        let c = b - id;
        for i in 0..N {
            for j in 0..N {
                assert!(c[i][j].abs() < 1e-10);
            }
        }
    }
}