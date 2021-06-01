#[macro_export]
macro_rules! w {
    ($($arg:tt)*) => { write!($($arg)*).unwrap(); }
}
#[macro_export]
macro_rules! wln {
    ($dst:expr $(, $($arg:tt)*)?) => {{
        writeln!($dst $(, $($arg)*)?).unwrap();
        #[cfg(debug_assertions)]
        $dst.flush().unwrap();
    }}
}
#[macro_export]
macro_rules! w_iter {
    ($dst:expr, $fmt:expr, $iter:expr, $delim:expr) => {{
        let mut first = true;
        for elem in $iter {
            if first {
                w!($dst, $fmt, elem);
                first = false;
            } else {
                w!($dst, concat!($delim, $fmt), elem);
            }
        }
    }};
    ($dst:expr, $fmt:expr, $iter:expr) => {
        w_iter!($dst, $fmt, $iter, " ")
    };
}
#[macro_export]
macro_rules! w_iter_ln {
    ($dst:expr, $($t:tt)*) => {{
        w_iter!($dst, $($t)*);
        wln!($dst);
    }}
}
#[macro_export]
macro_rules! e {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        eprint!($($t)*)
    }
}
#[macro_export]
macro_rules! eln {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($t)*)
    }
}
#[doc(hidden)]
#[macro_export]
macro_rules! __tstr {
    ($h:expr $(, $t:expr)+) => { concat!(__tstr!($($t),+), ", ", __tstr!(@)) };
    ($h:expr) => { concat!(__tstr!(), " ",  __tstr!(@)) };
    () => { "\x1B[94m[{}:{}]\x1B[0m" };
    (@) => { "\x1B[1;92m{}\x1B[0m = {:?}" }
}
#[macro_export]
macro_rules! d {
    ($($a:expr),*) => { eln!(__tstr!($($a),*), file!(), line!(), $(stringify!($a), $a),*) };
}
