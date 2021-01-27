use std::{
    io::prelude::*,
    iter::FromIterator,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    str,
};

pub trait Input {
    fn bytes(&mut self) -> &[u8];
    fn str(&mut self) -> &str {
        str::from_utf8(self.bytes()).unwrap()
    }
    fn input<T: InputItem>(&mut self) -> T {
        T::input(self)
    }
    fn iter<T: InputItem>(&mut self) -> Iter<T, Self> {
        Iter(self, PhantomData)
    }
    fn collect<T: InputItem, B: FromIterator<T>>(&mut self, n: usize) -> B {
        self.iter().take(n).collect()
    }
    fn map<T: InputItem, U, F: FnMut(T) -> U, B: FromIterator<U>>(
        &mut self,
        n: usize,
        f: F,
    ) -> B {
        self.iter().take(n).map(f).collect()
    }
}
impl<I: Input> Input for &mut I {
    fn bytes(&mut self) -> &[u8] {
        (**self).bytes()
    }
}
pub struct KInput<R> {
    src: R,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}
impl<R: Read> KInput<R> {
    pub fn new(src: R) -> Self {
        Self {
            src,
            buf: vec![0; 1 << 16],
            pos: 0,
            len: 0,
        }
    }
    fn read(&mut self) -> usize {
        if self.pos > 0 {
            self.buf.copy_within(self.pos..self.len, 0);
            self.len -= self.pos;
            self.pos = 0;
        } else if self.len >= self.buf.len() {
            self.buf.resize(2 * self.buf.len(), 0);
        }
        let n = self.src.read(&mut self.buf[self.len..]).unwrap();
        self.len += n;
        n
    }
}
impl<R: Read> Input for KInput<R> {
    fn bytes(&mut self) -> &[u8] {
        loop {
            while let Some(d) = self.buf[self.pos..self.len]
                .iter()
                .position(u8::is_ascii_whitespace)
            {
                let p = self.pos;
                self.pos += d + 1;
                if d > 0 {
                    return &self.buf[p..p + d];
                }
            }
            if self.read() == 0 {
                return &self.buf[mem::replace(&mut self.pos, self.len)..self.len];
            }
        }
    }
}
pub struct Iter<'a, T, I: ?Sized>(&'a mut I, PhantomData<*const T>);
impl<'a, T: InputItem, I: Input + ?Sized> Iterator for Iter<'a, T, I> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        Some(self.0.input())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (!0, None)
    }
}
pub trait InputItem: Sized {
    fn input<I: Input + ?Sized>(src: &mut I) -> Self;
}
impl InputItem for Vec<u8> {
    fn input<I: Input + ?Sized>(src: &mut I) -> Self {
        src.bytes().to_owned()
    }
}
macro_rules! from_str {
    ($($T:ty)*) => {$(
        impl InputItem for $T {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                src.str().parse::<$T>().unwrap()
            }
        }
    )*}
}
from_str!(String char bool f32 f64);
macro_rules! parse_int {
    ($($I:ty: $U:ty)*) => {$(
        impl InputItem for $I {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                let f = |s: &[u8]| s.iter().fold(0, |x, b| 10 * x + (b & 0xf) as $I);
                let s = src.bytes();
                if let Some((&b'-', t)) = s.split_first() { -f(t) } else { f(s) }
            }
        }
        impl InputItem for $U {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                src.bytes().iter().fold(0, |x, b| 10 * x + (b & 0xf) as $U)
            }
        }
    )*}
}
parse_int!(isize:usize i8:u8 i16:u16 i32:u32 i64:u64 i128:u128);
macro_rules! tuple {
    ($H:ident $($T:ident)*) => {
        impl<$H: InputItem, $($T: InputItem),*> InputItem for ($H, $($T),*) {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                ($H::input(src), $($T::input(src)),*)
            }
        }
        tuple!($($T)*);
    };
    () => {}
}
tuple!(A B C D E F G);
macro_rules! array {
    ($($N:literal)*) => {$(
        impl<T: InputItem> InputItem for [T; $N] {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                unsafe {
                    let mut arr: [MaybeUninit<T>; $N] = MaybeUninit::uninit().assume_init();
                    for elem in &mut arr {
                        *elem = MaybeUninit::new(src.input());
                    }
                    mem::transmute_copy(&arr)
                }
            }
        }
    )*}
}
array!(1 2 3 4 5 6 7 8);