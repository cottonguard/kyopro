mod pcg;
mod xoshiro;

pub use pcg::Pcg;
pub use xoshiro::*;

pub trait Rng {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn gen<T: Uniform>(&mut self) -> T {
        T::sample(self)
    }
    fn range<T: Uniform>(&mut self, l: T, r: T) -> T {
        T::range(self, l, r)
    }
    fn range_inclusive<T: Uniform>(&mut self, l: T, r: T) -> T {
        T::range_inclusive(self, l, r)
    }
    fn gen_bool(&mut self, p: f64) -> bool {
        if p >= 1. {
            return true;
        }
        self.next_u64() < (2.0f64.powi(64) * p) as u64
    }
    fn standard_normal<T: SampleFloat>(&mut self) -> T {
        T::standard_normal(self)
    }
    fn normal<T: SampleFloat>(&mut self, mean: T, sd: T) -> T {
        T::normal(self, mean, sd)
    }
    fn exp<T: SampleFloat>(&mut self, lambda: T) -> T {
        T::exp(self, lambda)
    }
    fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            a.swap(self.range_inclusive(0, i), i);
        }
    }
    fn partial_shuffle<'a, T>(&mut self, a: &'a mut [T], n: usize) -> (&'a mut [T], &'a mut [T]) {
        let n = n.min(a.len());
        for i in 0..n {
            a.swap(i, self.range(i, a.len()));
        }
        a.split_at_mut(n)
    }
    fn choose<'a, T>(&mut self, a: &'a [T]) -> &'a T {
        &a[self.range(0, a.len())]
    }
    fn choose_mut<'a, T>(&mut self, a: &'a mut [T]) -> &'a mut T {
        &mut a[self.range(0, a.len())]
    }
}
pub trait Uniform {
    fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
    fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
}
pub trait SampleFloat {
    fn standard_normal<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn normal<T: Rng + ?Sized>(rand: &mut T, mean: Self, sd: Self) -> Self;
    fn exp<T: Rng + ?Sized>(rand: &mut T, lambda: Self) -> Self;
}
macro_rules! int_impl {
    ($($type:ident),*) => {$(
        impl Uniform for $type {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                if 8 * std::mem::size_of::<Self>() <= 32 {
                    rand.next_u32() as $type
                } else {
                    rand.next_u64() as $type
                }
            }
            fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l < r);
                Self::range_inclusive(rand, l, r - 1)
            }
            fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                if 8 * std::mem::size_of::<Self>() <= 32 {
                    let d = (r - l) as u32;
                    let mask = if d == 0 { 0 } else { !0 >> d.leading_zeros() };
                    loop {
                        let x = rand.next_u32() & mask;
                        if x <= d {
                            return l + d as $type;
                        }
                    }
                } else {
                    let d = (r - l) as u64;
                    let mask = if d == 0 { 0 } else { !0 >> d.leading_zeros() };
                    loop {
                        let x = rand.next_u64() & mask;
                        if x <= d {
                            return l + d as $type;
                        }
                    }
                }
            }
        }
    )*};
}
int_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
macro_rules! float_impl {
    ($($fty:ident, $uty:ident, $fract:expr);*) => {$(
        impl Uniform for $fty {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let x: $uty = rand.gen();
                let bits = 8 * std::mem::size_of::<$fty>();
                let prec = $fract + 1;
                (x >> (bits - prec)) as $fty / ((1 as $uty) << prec) as $fty
            }
            fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                l + Self::sample(rand) / (r - l)
            }
            fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                Self::range(rand, l, r + std::$fty::EPSILON)
            }
        }
        impl SampleFloat for $fty {
            fn standard_normal<T: Rng + ?Sized>(rand: &mut T) -> Self {
                Self::exp(0.5).sqrt() * (2. * std::$fty::consts::PI * Self::sample(rand)).cos()
            }
            fn normal<T: Rng + ?Sized>(rand: &mut T, mean: Self, sd: Self) -> Self {
                sd * Self::standard_normal(rand) + mean
            }
            fn exp<T: Rng + ?Sized>(rand: &mut T, lambda: Self) -> Self {
                -1. / lambda * Self::sample(rand).ln()
            }
        }
    )*}
}
float_impl!(f32, u32, 23; f64, u64, 52);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_test() {
        let mut rand = Pcg::seed_from_u64(0);

        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        for _ in 0..1000 {
            let x: f32 = rand.gen();
            min = min.min(x);
            max = max.max(x);
            assert!(0. <= x);
            assert!(x < 1.);
        }
        assert!(min < 0.05);
        assert!(max > 0.95);

        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        for _ in 0..1000 {
            let x: f64 = rand.gen();
            min = min.min(x);
            max = max.max(x);
            assert!(0. <= x);
            assert!(x < 1.);
        }
        assert!(min < 0.05);
        assert!(max > 0.95);

        for &(d, u) in &[
            (0, 1),
            (0, 32),
            (0, 31),
            (0, 33),
            (10, 20),
            (16, 32),
            (-20, -10),
            (-20, 20),
        ] {
            for _ in 0..1000 {
                let x = rand.range(d, u);
                assert!(d <= x);
                assert!(x < u);

                let x = rand.range_inclusive(d, u);
                assert!(d <= x);
                assert!(x <= u);
            }
        }
    }
}
