trait OrdExt: Ord + Sized {
    fn chmin(&mut self, x: Self) -> bool {
        let c = x < *self;
        if c {
            *self = x;
        }
        c
    }
    fn chmax(&mut self, x: Self) -> bool {
        let c = x > *self;
        if c {
            *self = x;
        }
        c
    }
}
impl<T: Ord> OrdExt for T {}

pub fn pow<T, F: Fn(&T, &T) -> T>(mut x: T, mut k: i64, init: T, f: F) -> T {
    let mut y = init;
    while k > 0 {
        if k & 1 == 1 {
            y = f(&x, &y);
        }
        x = f(&x, &x);
        k >>= 1;
    }
    y
}

use std::{cell::UnsafeCell, sync::Once};
pub struct OnceInit<T> {
    value: UnsafeCell<Option<T>>,
    once: Once,
}
unsafe impl<T> Sync for OnceInit<T> {}
impl<T> OnceInit<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            once: Once::new(),
        }
    }
    pub fn set(&self, value: T) {
        self.once.call_once(|| unsafe {
            let v = &mut *self.value.get();
            if v.is_none() {
                *v = Some(value);
            }
        });
    }
    pub fn get(&self) -> Option<&T> {
        unsafe { (*self.value.get()).as_ref() }
    }
}
impl<T> std::ops::Deref for OnceInit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get().unwrap()
    }
}

#[derive(Clone, Copy, Debug, Hash)]
struct CmpKey<T, U>(T, U);
impl<T: PartialEq, U> PartialEq for CmpKey<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Eq, U> Eq for CmpKey<T, U> {}
impl<T: PartialOrd, U> PartialOrd for CmpKey<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Ord, U> Ord for CmpKey<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Clone, Copy, PartialOrd, Debug)]
struct OrdF32(f32);
impl PartialEq for OrdF32 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}
impl Eq for OrdF32 {}
impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("cmp failed")
    }
}
