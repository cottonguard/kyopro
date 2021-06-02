mod pcg;
mod xoshiro;

pub use pcg::Pcg;
pub use xoshiro::*;

pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
}
pub trait Rng: RngCore {
    fn gen<T: Sample>(&mut self) -> T {
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
    fn open01<T: SampleFloat>(&mut self) -> T {
        T::open01(self)
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
impl<T: RngCore> Rng for T {}
pub trait Sample {
    fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self;
}
pub trait Uniform {
    fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
    fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
}
pub trait SampleFloat {
    fn open01<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn standard_normal<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn normal<T: Rng + ?Sized>(rand: &mut T, mean: Self, sd: Self) -> Self;
    fn exp<T: Rng + ?Sized>(rand: &mut T, lambda: Self) -> Self;
}
macro_rules! int_impl {
    ($($type:ident),*) => {$(
        impl Sample for $type {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                if 8 * std::mem::size_of::<Self>() <= 32 {
                    rand.next_u32() as $type
                } else {
                    rand.next_u64() as $type
                }
            }
        }
        impl Uniform for $type {
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
    ($($fty:ident, $uty:ident, $fract:expr, $exp_bias:expr);*) => {$(
        impl Sample for $fty {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let x: $uty = rand.gen();
                let bits = 8 * std::mem::size_of::<$fty>();
                let prec = $fract + 1;
                let scale = 1. / ((1 as $uty) << prec) as $fty;
                scale * (x >> (bits - prec)) as $fty
            }
        }
        impl Uniform for $fty {
            fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                l + Self::sample(rand) / (r - l)
            }
            fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                Self::range(rand, l, r)
            }
        }
        impl SampleFloat for $fty {
            fn open01<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let x: $uty = rand.gen();
                let bits = 8 * std::mem::size_of::<$fty>();
                let exp = $exp_bias << $fract;
                $fty::from_bits(exp | (x >> (bits - $fract))) - (1. - std::$fty::EPSILON / 2.)
            }
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
float_impl!(f32, u32, 23, 127; f64, u64, 52, 1023);
impl Sample for bool {
    fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
        (rand.next_u32() as i32) >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! float_test {
        ($ty:ident, $method:ident, $lo:expr, $hi:expr, $min:expr, $max:expr, $mean:expr) => {{
            let mut rand = Xoshiro::seed_from_u64(0);
            let mut min = $ty::INFINITY;
            let mut max = -$ty::INFINITY;
            let mut sum = 0.;
            for _ in 0..1000 {
                let x: $ty = rand.$method();
                min = min.min(x);
                max = max.max(x);
                sum += x;
                assert!($lo(x));
                assert!($hi(x));
            }
            assert!($min(min));
            assert!($max(max));
            let mean = sum / 1000.;
            assert!($mean(mean));
        }};
    }

    #[test]
    fn random_test() {
        let mut rand = Pcg::seed_from_u64(0);

        float_test!(
            f32,
            gen,
            |x| 0. <= x,
            |x| x < 1.,
            |x| x < 0.05,
            |x| 0.95 < x,
            |x| 0.45 < x && x < 0.55
        );
        float_test!(
            f64,
            gen,
            |x| 0. <= x,
            |x| x < 1.,
            |x| x < 0.05,
            |x| 0.95 < x,
            |x| 0.45 < x && x < 0.55
        );
        float_test!(
            f32,
            open01,
            |x| 0. < x,
            |x| x < 1.,
            |x| x < 0.05,
            |x| 0.95 < x,
            |x| 0.45 < x && x < 0.55
        );
        float_test!(
            f64,
            open01,
            |x| 0. < x,
            |x| x < 1.,
            |x| x < 0.05,
            |x| 0.95 < x,
            |x| 0.45 < x && x < 0.55
        );
        float_test!(
            f64,
            standard_normal,
            |_| true,
            |_| true,
            |_| true,
            |_| true,
            |x| -0.05 < x && x < 0.05
        );

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
