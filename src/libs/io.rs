use std::{io::prelude::*, mem};
pub trait Input {
    fn bytes(&mut self) -> &[u8];
    fn str(&mut self) -> &str {
        std::str::from_utf8(self.bytes()).unwrap()
    }
    fn input<T: InputParse>(&mut self) -> T {
        T::input(self)
    }
    fn iter<T: InputParse>(&mut self) -> Iter<T, Self> {
        Iter(self, std::marker::PhantomData)
    }
    fn seq<T: InputParse, B: std::iter::FromIterator<T>>(&mut self, n: usize) -> B {
        self.iter().take(n).collect()
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
}
impl<R: Read> Input for KInput<R> {
    fn bytes(&mut self) -> &[u8] {
        loop {
            while let Some(delim) = self.buf[self.pos..self.len]
                .iter()
                .position(|b| b.is_ascii_whitespace())
            {
                let p = self.pos;
                self.pos += delim + 1;
                if delim > 0 {
                    return &self.buf[p..p + delim];
                }
            }
            if self.read() == 0 {
                return &self.buf[mem::replace(&mut self.pos, self.len)..self.len];
            }
        }
    }
}
impl<R: Read> KInput<R> {
    fn read(&mut self) -> usize {
        if self.pos > 0 {
            self.buf.copy_within(self.pos..self.len, 0);
            self.len -= self.pos;
            self.pos = 0;
        } else if self.len >= self.buf.len() {
            self.buf.resize(2 * self.buf.len(), 0);
        }
        let read = self.src.read(&mut self.buf[self.len..]).unwrap();
        self.len += read;
        read
    }
}
pub struct Iter<'a, T, I: ?Sized>(&'a mut I, std::marker::PhantomData<*const T>);
impl<'a, T: InputParse, I: Input + ?Sized> Iterator for Iter<'a, T, I> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        Some(self.0.input())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (!0, None)
    }
}
pub trait InputParse: Sized {
    fn input<I: Input + ?Sized>(src: &mut I) -> Self;
}
impl InputParse for Vec<u8> {
    fn input<I: Input + ?Sized>(src: &mut I) -> Self {
        src.bytes().to_owned()
    }
}
macro_rules! from_str_impl {
    { $($T:ty)* } => {
        $(impl InputParse for $T {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                src.str().parse::<$T>().unwrap()
            }
        })*
    }
}
from_str_impl! { String char bool f32 f64 }
macro_rules! parse_int_impl {
    { $($I:ty: $U:ty)* } => {
        $(impl InputParse for $I {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                let f = |s: &[u8]| s.iter().fold(0, |x, b| 10 * x + (b & 0xf) as $I);
                let s = src.bytes();
                if let Some((&b'-', t)) = s.split_first() { -f(t) } else { f(s) }
            }
        }
        impl InputParse for $U {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                src.bytes().iter().fold(0, |x, b| 10 * x + (b & 0xf) as $U)
            }
        })*
    };
}
parse_int_impl! { isize:usize i8:u8 i16:u16 i32:u32 i64:u64 i128:u128 }
macro_rules! tuple_impl {
    ($H:ident $($T:ident)*) => {
        impl<$H: InputParse, $($T: InputParse),*> InputParse for ($H, $($T),*) {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                ($H::input(src), $($T::input(src)),*)
            }
        }
        tuple_impl!($($T)*);
    };
    () => {}
}
tuple_impl!(A B C D E F G);
macro_rules! array_impl {
    { $($N:literal)* } => {
        $(impl<T: InputParse> InputParse for [T; $N] {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                let mut arr = mem::MaybeUninit::uninit();
                unsafe {
                    let ptr = arr.as_mut_ptr() as *mut T;
                    for i in 0..$N {
                        ptr.add(i).write(src.input());
                    }
                    arr.assume_init()
                }
            }
        })*
    };
}
array_impl! { 1 2 3 4 5 6 7 8 }
#[macro_export]
macro_rules! kdbg {
    ($($v:expr),*) => {
        if cfg!(debug_assertions) { dbg!($($v),*) } else { ($($v),*) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input() {
        let s = b"123 -3230     abcd \r\n  efgh\n12.34";
        let mut kin = KInput::new(s as &[u8]);
        assert_eq!(kin.input::<(u32, i32)>(), (123, -3230));
        assert_eq!(kin.input::<Vec<u8>>(), b"abcd");
        assert_eq!(kin.input::<String>(), "efgh");
        assert!((kin.input::<f64>() - 12.34).abs() < 1e-15);
        assert_eq!(kin.bytes(), b"");
    }

    #[test]
    fn seq() {
        let s = b"1  2 3\n4 5";
        let mut kin = KInput::new(s as &[u8]);
        let a: Vec<i32> = kin.seq(5);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }
}
