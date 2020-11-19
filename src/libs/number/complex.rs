#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Complex(pub f64, pub f64);
impl Complex {
    pub fn polar(r: f64, theta: f64) -> Self {
        Self(theta.cos(), theta.sin()).scale(r)
    }
    pub fn conj(self) -> Self {
        Self(self.1, self.0)
    }
    pub fn inv(self) -> Self {
        let d = self.norm();
        Self(self.0 / d, -self.1 / d)
    }
    pub fn norm(self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }
    pub fn abs(self) -> f64 {
        self.0.hypot(self.1)
    }
    pub fn arg(self) -> f64 {
        self.1.atan2(self.0)
    }
    pub fn exp(self) -> Self {
        let (s, c) = self.1.sin_cos();
        Self(c, s).scale(self.0.exp())
    }
    pub fn powf(self, s: f64) -> Self {
        Self::polar(self.abs().powf(s), s * self.arg())
    }
    pub fn normalize(self) -> Self {
        self.scale(1. / self.abs())
    }
    pub fn scale(self, s: f64) -> Self {
        Self(s * self.0, s * self.1)
    }
    pub fn dot(self, other: Self) -> f64 {
        (self * other).0
    }
    pub fn cross(self, other: Self) -> f64 {
        (self.conj() * other).1
    }
    pub fn project_onto(self, onto: Self) -> Self {
        onto.scale(self.dot(onto) / self.norm())
    }
    pub fn reflect(self, normal: Self) -> Self {
        self.project_onto(normal).scale(2.) - self
    }
}
use std::ops::*;
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let Self(re1, im1) = self;
        let Self(re2, im2) = other;
        Self(re1 * re2 - im1 * im2, im1 * re2 + re1 * im2)
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let Self(re1, im1) = self;
        let Self(re2, im2) = other;
        Self(re1 * re2 + im1 * im2, im1 * re2 - re1 * im2).scale(other.norm())
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
impl From<f64> for Complex {
    fn from(x: f64) -> Self {
        Self(x, 0.)
    }
}