#![allow(unused_imports, unused_macros)]

use kyoproio::*;
use std::{
    collections::*,
    io::{self, prelude::*},
    iter,
    mem::{replace, swap},
};

fn run<I: Input, O: Write>(mut kin: I, mut out: O) {
    let (n, q, c): (usize, usize, i64) = kin.input();
    let mut g = LabeledGraph::builder(n + 1);
    for (u, v, l) in kin.iter::<(usize, usize, i64)>().take(n - 1) {
        g.edge(u, v, l);
        g.edge(v, u, l);
    }
    let g = g.build();
    let x: Vec<usize> = kin.seq(q);
    let hld = Hld::new(&g, 1);
    let mut dist = vec![vec![1 << 60; n + 1]; n + 1];
    for u in 1..=n {
        dist[u][u] = 0;
        dfs(&g, &mut dist[u], u, 0);
    }
    let mut sum = vec![0; q];
    for i in 1..q {
        sum[i] = sum[i - 1] + dist[x[i - 1]][x[i]];
    }
    let mut dp = vec![0; q];
    for i in 1..q {
        dp[i] = dp[i - 1] + dist[x[i - 1]][x[i]];
        for j in 1..i {
            let (u, v, w) = (x[j - 1], x[j], x[i]);
            let auv = hld.lca(u, v);
            let auw = hld.lca(u, w);
            let avw = hld.lca(v, w);
            let z = if auw == w && avw == w {
                //   w
                //   |
                //   z
                //  / \
                // u   v
                auv
            } else if auv == u && auw == u {
                avw
            } else if auv == v && avw == v {
                auw
            } else {
                w
            };
            dp[i] = dp[i].min(dp[j - 1] + (sum[i - 1] - sum[j - 1]) + c + dist[z][w]);
        }
    }
    // eprintln!("{:?}", sum);
    // eprintln!("{:?}", dp);
    outln!(out, dp[q - 1]);
}

fn dfs(g: &LabeledGraph<i64>, dist: &mut [i64], u: usize, p: usize) {
    for &(v, c) in &g[u] {
        if v != p {
            dist[v] = dist[u] + c;
            dfs(g, dist, v, u);
        }
    }
}

pub struct Graph(LabeledGraph<()>);
impl Graph {
    pub fn builder(n: usize) -> GraphBuilder {
        GraphBuilder(LabeledGraph::builder(n))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl std::ops::Index<usize> for Graph {
    type Output = [usize];
    fn index(&self, u: usize) -> &Self::Output {
        unsafe { std::mem::transmute(self.0.index(u)) }
    }
}
pub struct GraphBuilder(LabeledGraphBuilder<()>);
impl GraphBuilder {
    pub fn edge(&mut self, u: usize, v: usize) {
        self.0.edge(u, v, ());
    }
    pub fn build(&mut self) -> Graph {
        Graph(self.0.build())
    }
}
pub struct LabeledGraph<T> {
    edges: Box<[(usize, T)]>,
    heads: Box<[usize]>,
}
impl<T: Clone> LabeledGraph<T> {
    pub fn builder(n: usize) -> LabeledGraphBuilder<T> {
        LabeledGraphBuilder {
            nodes: Vec::new(),
            heads: vec![!0; n],
        }
    }
    pub fn len(&self) -> usize {
        self.heads.len() - 1
    }
}
impl<T> std::ops::Index<usize> for LabeledGraph<T> {
    type Output = [(usize, T)];
    fn index(&self, u: usize) -> &Self::Output {
        &self.edges[self.heads[u]..self.heads[u + 1]]
    }
}
pub struct LabeledGraphBuilder<T> {
    nodes: Vec<((usize, T), usize)>,
    heads: Vec<usize>,
}
impl<T: Clone> LabeledGraphBuilder<T> {
    pub fn edge(&mut self, u: usize, v: usize, l: T) {
        self.nodes.push(((v, l), self.heads[u]));
        self.heads[u] = self.nodes.len() - 1;
    }
    pub fn build(&mut self) -> LabeledGraph<T> {
        let mut edges = Vec::with_capacity(self.nodes.len());
        let mut heads = Vec::with_capacity(self.heads.len() + 1);
        for &(mut h) in &self.heads {
            heads.push(edges.len());
            while let Some((e, next)) = self.nodes.get(h) {
                edges.push(e.clone());
                h = *next;
            }
        }
        heads.push(edges.len());
        LabeledGraph {
            edges: edges.into(),
            heads: heads.into(),
        }
    }
}

#[derive(Debug)]
pub struct Hld {
    head: Vec<usize>,
    par: Vec<usize>,
    tab: Vec<usize>,
}
pub type G = LabeledGraph<i64>;
impl Hld {
    pub fn new(g: &G, root: usize) -> Self {
        let mut heavy = vec![!0; g.len()];
        Self::dfs_heavy(g, &mut heavy, &mut vec![1; g.len()], root, !0);
        let mut hld = Hld {
            head: Vec::with_capacity(g.len()),
            par: Vec::with_capacity(g.len()),
            tab: vec![0; g.len()],
        };
        hld.dfs_build(g, &heavy, root, !0, root);
        hld
    }
    fn dfs_heavy(g: &G, heavy: &mut [usize], size: &mut [usize], u: usize, p: usize) {
        let mut max = 0;
        for v in g[u].iter().map(|a| a.0).filter(|&v| v != p) {
            Self::dfs_heavy(g, heavy, size, v, u);
            if size[v] > max {
                max = size[v];
                heavy[u] = v;
            }
            size[u] += size[v];
        }
    }
    fn dfs_build(&mut self, g: &G, heavy: &[usize], u: usize, p: usize, h: usize) {
        self.tab[u] = self.head.len();
        self.head.push(h);
        self.par.push(p);
        if heavy[u] == !0 {
            return;
        }
        self.dfs_build(g, heavy, heavy[u], u, h);
        for v in g[u].iter().map(|a| a.0).filter(|&v| v != p && v != heavy[u]) {
            self.dfs_build(g, heavy, v, u, v);
        }
    }
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.tab[u] > self.tab[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[self.tab[u]] == self.head[self.tab[v]] {
                return u;
            }
            v = self.par[self.tab[self.head[self.tab[v]]]];
        }
    }
}


// -----------------------------------------------------------------------------
fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            run(
                KInput::new(io::stdin().lock()),
                io::BufWriter::new(io::stdout().lock()),
            )
        })?
        .join()
        .unwrap();
    Ok(())
}

// -----------------------------------------------------------------------------
pub mod kyoproio {
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
}
