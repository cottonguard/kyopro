pub struct FenwickTree<T, F, Z> {
    a: Vec<T>,
    f: F,
    z: Z,
}
impl<T, F: Fn(&T, &T) -> T, Z: Fn() -> T> FenwickTree<T, F, Z> {
    pub fn new(n: usize, z: Z, f: F) -> Self {
        Self {
            a: (0..=n).map(|_| z()).collect(),
            f,
            z,
        }
    }
    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i < self.a.len() {
            self.a[i] = (self.f)(&self.a[i], &x);
            i += i & (!i + 1);
        }
    }
    // [0, i)
    pub fn sum(&self, mut i: usize) -> T {
        let mut s = (self.z)();
        while i > 0 {
            s = (self.f)(&self.a[i], &s);
            i -= i & (!i + 1);
        }
        s
    }
    pub fn reset(&mut self) {
        for a in &mut self.a {
            *a = (self.z)();
        }
    }
}
