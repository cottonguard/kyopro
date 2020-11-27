pub type Mint = ModInt<Mod998244353>;
pub fn mint(x: i32) -> Mint {
    ModInt::new(x)
}
pub trait Modulo {
    fn modulo() -> i32;
}
macro_rules! modulo_impl {
    ($($Type:ident $val:tt)*) => {
        $(pub struct $Type;
        impl Modulo for $Type {
            fn modulo() -> i32 {
                $val
            }
        })*
    };
}
modulo_impl!(Mod998244353 998244353 Mod1e9p7 1000000007);
use std::sync::atomic;
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
        assert_ne!(rhs.get(), 0);
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
        }
        impl<M: Modulo> ops::$Op<&Self> for ModInt<M> {
            type Output = Self;
            fn $op(self, rhs: &Self) -> Self {
                self.$op(*rhs)
            }
        }
        impl<M: Modulo> ops::$Op<ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: ModInt<M>) -> ModInt<M> {
                (*self).$op(rhs)
            }
        }
        impl<M: Modulo> ops::$Op<&ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: &ModInt<M>) -> ModInt<M> {
                (*self).$op(*rhs)
            }
        }
        impl<M: Modulo> ops::$OpAssign<&ModInt<M>> for ModInt<M> {
            fn $op_assign(&mut self, rhs: &ModInt<M>) {
                self.$op_assign(*rhs);
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
pub trait Pow<T> {
    fn pow(self, n: T) -> Self;
}
impl<M: Modulo> Pow<u32> for ModInt<M> {
    fn pow(mut self, mut n: u32) -> Self {
        let mut y = Self::new(1);
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
macro_rules! mod_int_pow_impl {
    ($($T:ident)*) => {
        $(impl<M: Modulo> Pow<$T> for ModInt<M> {
            fn pow(self, n: $T) -> Self {
                self.pow(n.rem_euclid(M::modulo() as $T - 1) as u32)
            }
        })*
    };
}
mod_int_pow_impl!(isize i32 i64 usize u64);
macro_rules! mod_int_from_impl {
    ($($T:ident)*) => {
        $(impl<M: Modulo> From<$T> for ModInt<M> {
            fn from(x: $T) -> Self {
                Self::new(x.rem_euclid(M::modulo() as $T) as i32)
            }
        })*
    }
}
mod_int_from_impl!(isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128);
impl<M> Copy for ModInt<M> {}
impl<M> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: Modulo> Default for ModInt<M> {
    fn default() -> Self {
        Self::new(0)
    }
}
impl<M> std::cmp::PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<M> std::cmp::Eq for ModInt<M> {}
impl<M> std::cmp::PartialOrd for ModInt<M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<M> std::cmp::Ord for ModInt<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<M> std::hash::Hash for ModInt<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<M> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<M> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("ModInt(")?;
        self.0.fmt(f)?;
        f.pad(")")
    }
}
