pub type Mint = ModInt<Mod998244353>;
pub fn mint<T: Into<i32>>(x: T) -> ModInt<Mod998244353> {
    ModInt::new(x.into())
}
pub trait Modulo: Copy {
    fn modulo() -> i32;
}
macro_rules! modulo_impl {
    ($($Type:ident $val:tt)*) => {
        $(#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
        pub struct $Type;
        impl Modulo for $Type {
            fn modulo() -> i32 {
                $val
            }
        })*
    };
}
modulo_impl!(Mod998244353 998244353 Mod1e9p7 1000000007);
use std::sync::atomic;
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct VarMod;
static VAR_MOD: atomic::AtomicI32 = atomic::AtomicI32::new(0);
pub fn set_var_mod(m: i32) {
    VAR_MOD.store(m, atomic::Ordering::Relaxed);
}
impl Modulo for VarMod {
    fn modulo() -> i32 {
        VAR_MOD.load(atomic::Ordering::Relaxed)
    }
}
use std::{fmt, marker::PhantomData, ops};
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModInt<M>(i32, PhantomData<M>);
impl<M: Modulo> ModInt<M> {
    pub fn new(x: i32) -> Self {
        debug_assert!(x < M::modulo());
        Self(x, PhantomData)
    }
    pub fn normalize(self) -> Self {
        if self.0 < M::modulo() && 0 <= self.0 {
            self
        } else {
            Self::new(self.0.rem_euclid(M::modulo()))
        }
    }
    pub fn get(self) -> i32 {
        self.0
    }
    pub fn inv(self) -> Self {
        self.pow(M::modulo() - 2)
    }
    pub fn pow(self, mut n: i32) -> Self {
        while n < 0 {
            n += M::modulo() - 1;
        }
        let mut x = self;
        let mut y = Self::new(1);
        while n > 0 {
            if n % 2 == 1 {
                y *= x;
            }
            x *= x;
            n /= 2;
        }
        y
    }
    pub fn half(self) -> Self {
        Self::new(self.0 / 2 + self.0 % 2 * ((M::modulo() + 1) / 2))
    }
    pub fn modulo() -> i32 {
        M::modulo()
    }
}
impl<M: Modulo> ops::Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(if self.0 == 0 { 0 } else { M::modulo() - self.0 })
    }
}
impl<M: Modulo> ops::AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= M::modulo() {
            self.0 -= M::modulo();
        }
    }
}
impl<M: Modulo> ops::SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        if self.0 < 0 {
            self.0 += M::modulo();
        }
    }
}
impl<M: Modulo> ops::MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 as u32 as u64 * rhs.0 as u32 as u64 % M::modulo() as u32 as u64) as i32;
    }
}
impl<M: Modulo> ops::DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        assert_ne!(rhs.0, 0);
        *self *= rhs.inv();
    }
}
macro_rules! op_impl {
    ($($Op:ident $op:ident $OpAssign:ident $op_assign:ident)*) => {
        $(impl<M: Modulo> ops::$Op for ModInt<M> {
            type Output = Self;
            fn $op(self, rhs: Self) -> Self {
                let mut res = self;
                ops::$OpAssign::$op_assign(&mut res, rhs);
                res
            }
        })*
    };
}
op_impl! {
    Add add AddAssign add_assign
    Sub sub SubAssign sub_assign
    Mul mul MulAssign mul_assign
    Div div DivAssign div_assign
}
impl<M: Modulo> std::iter::Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(0), |x, y| x + y)
    }
}
impl<M: Modulo> std::iter::Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |x, y| x * y)
    }
}
impl<M: Modulo> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<M: Modulo> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("ModInt(")?;
        self.0.fmt(f)?;
        f.pad(")")
    }
}