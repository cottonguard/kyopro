#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Rational(i64, i64);
impl Rational {
    pub fn new(nom: i64, den: i64) -> Self {
        Self(nom, den).reduced()
    }
    pub fn inv(self) -> Self {
        if self.0 >= 0 {
            Self(self.1, self.0)
        } else {
            Self(-self.1, -self.0)
        }
    }
    fn reduced(self) -> Self {
        let Self(nom, den) = self;
        let d = gcd(nom.abs(), den.abs());
        if den >= 0 {
            Self(nom / d, den / d)
        } else {
            Self(-nom / d, -den / d)
        }
        .checked()
    }
    fn checked(self) -> Self {
        assert_ne!(self, Self(0, 0));
        self
    }
}
use std::{cmp::Ordering, ops::*};
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == 0 && other.1 == 0 {
            self.0.cmp(&other.0)
        } else {
            (self.0 as i128 * other.1 as i128).cmp(&(self.1 as i128 * other.0 as i128))
        }
    }
}
impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, self.1)
    }
}
impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 * other.1 + self.1 * other.0, self.1 * other.1).reduced()
    }
}
impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + -other
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1).reduced()
    }
}
impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<i64> for Rational {
    fn from(x: i64) -> Self {
        Self(x, 1)
    }
}
impl From<Rational> for f64 {
    fn from(r: Rational) -> f64 {
        r.0 as f64 / r.1 as f64
    }
}
fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}