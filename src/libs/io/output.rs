use std::{io::prelude::*, mem::MaybeUninit, ptr, slice, str};

pub struct KOutput<W: Write> {
    dest: W,
    delim: bool,
}
impl<W: Write> KOutput<W> {
    pub fn new(dest: W) -> Self {
        Self { dest, delim: false }
    }
    pub fn bytes(&mut self, s: &[u8]) {
        self.dest.write_all(s).unwrap();
    }
    pub fn byte(&mut self, b: u8) {
        self.bytes(slice::from_ref(&b));
    }
    pub fn output<T: OutputItem>(&mut self, x: T) {
        if self.delim {
            self.byte(b' ');
        }
        self.delim = true;
        x.output(self);
    }
    pub fn ln(&mut self) {
        self.delim = false;
        self.byte(b'\n');
        self.flush_debug();
    }
    pub fn inner(&mut self) -> &mut W {
        &mut self.dest
    }
    pub fn seq<T: OutputItem, I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter.into_iter() {
            self.output(x);
        }
    }
    pub fn flush(&mut self) {
        self.dest.flush().unwrap();
    }
    pub fn flush_debug(&mut self) {
        if cfg!(debug_assertions) {
            self.flush();
        }
    }
}
pub trait OutputItem {
    fn output<W: Write>(self, dest: &mut KOutput<W>);
}
impl OutputItem for &str {
    fn output<W: Write>(self, dest: &mut KOutput<W>) {
        dest.bytes(self.as_bytes());
    }
}
impl OutputItem for char {
    fn output<W: Write>(self, dest: &mut KOutput<W>) {
        self.encode_utf8(&mut [0; 4]).output(dest);
    }
}
macro_rules! output_fmt {
    ($($T:ty)*) => {
        $(impl OutputItem for $T {
            fn output<W: Write>(self, dest: &mut KOutput<W>) {
                write!(dest.inner(), "{}", self).unwrap();
            }
        })*
    }
}
output_fmt!(f32 f64);
macro_rules! output_int {
    ($conv:ident; $U:ty; $($T:ty)*) => {
        $(impl OutputItem for $T {
            fn output<W: Write>(self, dest: &mut KOutput<W>) {
                let mut buf = MaybeUninit::<[u8; 20]>::uninit();
                unsafe {
                    let ptr = buf.as_mut_ptr() as *mut u8;
                    let ofs = $conv(self as $U, ptr, 20);
                    dest.bytes(slice::from_raw_parts(ptr.add(ofs), 20 - ofs));
                }
            }
        }
        impl OutputItem for &$T {
            fn output<W: Write>(self, dest: &mut KOutput<W>) {
                (*self).output(dest);
            }
        })*
    };
}
output_int!(i64_to_bytes; i64; isize i8 i16 i32 i64);
output_int!(u64_to_bytes; u64; usize u8 u16 u32 u64);
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
    ($out:expr, $($args:expr),*) => {{ $($out.output($args);)* }};
}
#[macro_export]
macro_rules! outln {
    ($out:expr) => { $out.ln(); };
    ($out:expr, $($args:expr),*) => {{
        out!($out, $($args),*);
        outln!($out);
    }}
}
