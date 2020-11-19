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

#[derive(Clone, Copy, Debug)]
pub struct Ordered<T, U>(pub T, pub U);
impl<T: Ord, U> PartialEq for Ordered<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: Ord, U> Eq for Ordered<T, U> {}
impl<T: Ord, U> PartialOrd for Ordered<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Ord, U> Ord for Ordered<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
