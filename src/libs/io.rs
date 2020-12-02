use std::{
    io::prelude::*,
    iter::FromIterator,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr, slice, str,
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
    fn seq<T: InputItem, B: FromIterator<T>>(&mut self, n: usize) -> B {
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
macro_rules! from_str_impl {
    { $($T:ty)* } => {
        $(impl InputItem for $T {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                src.str().parse::<$T>().unwrap()
            }
        })*
    }
}
from_str_impl! { String char bool f32 f64 }
macro_rules! parse_int_impl {
    { $($I:ty: $U:ty)* } => {
        $(impl InputItem for $I {
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
        })*
    };
}
parse_int_impl! { isize:usize i8:u8 i16:u16 i32:u32 i64:u64 i128:u128 }
macro_rules! tuple_impl {
    ($H:ident $($T:ident)*) => {
        impl<$H: InputItem, $($T: InputItem),*> InputItem for ($H, $($T),*) {
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
        $(impl<T: InputItem> InputItem for [T; $N] {
            fn input<I: Input + ?Sized>(src: &mut I) -> Self {
                let mut arr = MaybeUninit::uninit();
                let ptr = arr.as_mut_ptr() as *mut T;
                unsafe {
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
pub trait Output: Write + Sized {
    fn bytes(&mut self, buf: &[u8]) {
        self.write_all(buf).unwrap();
    }
    fn output<T: OutputItem>(&mut self, x: T) {
        x.output(self);
    }
    fn byte(&mut self, b: u8) {
        self.bytes(slice::from_ref(&b));
    }
    fn seq<T: OutputItem, I: IntoIterator<Item = T>>(&mut self, iter: I, delim: u8) {
        let mut iter = iter.into_iter();
        if let Some(x) = iter.next() {
            self.output(x);
            for x in iter {
                self.byte(delim);
                self.output(x);
            }
        }
    }
    fn flush_debug(&mut self) {
        if cfg!(debug_assertions) {
            self.flush().unwrap();
        }
    }
}
impl<W: Write + Sized> Output for W {}
pub trait OutputItem {
    fn output<O: Output>(self, dest: &mut O);
}
impl OutputItem for &str {
    fn output<O: Output>(self, dest: &mut O) {
        dest.bytes(self.as_bytes());
    }
}
macro_rules! output_int_impl {
    ($conv:ident; $U:ty; $($T:ty)*) => {
        $(impl OutputItem for $T {
            fn output<O: Output>(self, dest: &mut O) {
                let mut buf = MaybeUninit::<[u8; 20]>::uninit();
                unsafe {
                    let ptr = buf.as_mut_ptr() as *mut u8;
                    let ofs = $conv(self as $U, ptr, 20);
                    dest.bytes(slice::from_raw_parts(ptr.add(ofs), 20 - ofs));
                }
            }
        }
        impl OutputItem for &$T {
            fn output<O: Output>(self, dest: &mut O) {
                (*self).output(dest);
            }
        })*
    };
}
output_int_impl!(i64_to_bytes; i64; isize i8 i16 i32 i64);
output_int_impl!(u64_to_bytes; u64; usize u8 u16 u32 u64);
static DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";
unsafe fn i64_to_bytes(x: i64, buf: *mut u8, len: usize) -> usize {
    let (neg, x) = if x < 0 { (true, -x) } else { (false, x) };
    let mut i = u64_to_bytes(x as u64, buf, len);
    if neg {
        i -= 1;
        *buf.add(i) = b'-';
    }
    i
}
unsafe fn u64_to_bytes(mut x: u64, buf: *mut u8, len: usize) -> usize {
    let lut = DIGITS_LUT.as_ptr();
    let mut i = len;
    let mut two = |x| {
        i -= 2;
        ptr::copy_nonoverlapping(lut.add(2 * x), buf.add(i), 2);
    };
    while x >= 10000 {
        let rem = (x % 10000) as usize;
        two(rem % 100);
        two(rem / 100);
        x /= 10000;
    }
    let mut x = x as usize;
    if x >= 100 {
        two(x % 100);
        x /= 100;
    }
    if x >= 10 {
        two(x);
    } else {
        i -= 1;
        *buf.add(i) = x as u8 + b'0';
    }
    i
}
#[macro_export]
macro_rules! out {
    ($out:expr, $arg:expr) => {{
        $out.output($arg);
    }};
    ($out:expr, $first:expr, $($rest:expr),*) => {{
        $out.output($first);
        $out.ws();
        out!($out, $($rest),*);
    }}
}
#[macro_export]
macro_rules! outln {
    ($out:expr, $($args:expr),*) => {{
        out!($out, $($args),*);
        $out.byte(b'\n');
        $out.flush_debug();
    }}
}
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

    #[test]
    fn output_int() {
        let mut out = Vec::<u8>::new();
        let mut out_fmt = Vec::<u8>::new();
        let mut x = 0;
        for i in 1..10 {
            x = 10 * x + i;
            let y = if i % 3 == 0 { -x } else { x };
            out.output(&y);
            out_fmt.extend_from_slice(format!("{}", y).as_bytes());
        }
        // dbg!(String::from_utf8_lossy(&out));
        assert_eq!(out, out_fmt);
    }

    #[test]
    fn output_int_seq() {
        let a: Vec<_> = (-10..=10).collect();
        let mut out = Vec::<u8>::new();
        out.seq(&a, b' ');
        let mut out_fmt = Vec::new();
        for (i, x) in a.into_iter().enumerate() {
            if i > 0 {
                out_fmt.push(b' ');
            }
            out_fmt.extend_from_slice(format!("{}", x).as_bytes());
        }
        assert_eq!(out, out_fmt);
    }
}
