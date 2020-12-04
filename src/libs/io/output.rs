use std::{io::prelude::*, mem::MaybeUninit, ptr, slice, str};
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
impl OutputItem for char {
    fn output<O: Output>(self, dest: &mut O) {
        self.encode_utf8(&mut [0u8; 4]).output(dest);
    }
}
impl OutputItem for () {
    fn output<O: Output>(self, _dest: &mut O) {}
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
        $out.byte(b' ');
        out!($out, $($rest),*);
    }}
}
#[macro_export]
macro_rules! outln {
    ($out:expr) => {{
        $out.byte(b'\n');
        $out.flush_debug();
    }};
    ($out:expr, $($args:expr),*) => {{
        out!($out, $($args),*);
        outln!($out);
    }}
}
#[macro_export]
macro_rules! kdbg {
    ($($v:expr),*) => {
        if cfg!(debug_assertions) { dbg!($($v),*) } else { ($($v),*) }
    }
}
