use std::cell::RefCell;
pub struct Arena<T>(RefCell<(Vec<T>, Vec<Vec<T>>)>);
const CHUNK_CAP: usize = 128;
impl<T> Arena<T> {
    pub fn new() -> Self {
        Self(RefCell::new((Vec::with_capacity(CHUNK_CAP), Vec::new())))
    }
    pub fn alloc(&self, x: T) -> &mut T {
        let mut r = self.0.borrow_mut();
        if r.0.len() >= r.0.capacity() {
            let a = std::mem::replace(&mut r.0, Vec::with_capacity(CHUNK_CAP));
            r.1.push(a);
        }
        let i = r.0.len();
        r.0.push(x);
        unsafe { &mut *r.0.as_mut_ptr().add(i) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arena() {
        let arena = Arena::new();
        let mut refs = Vec::new();
        const N: usize = 1000;
        for i in 0..N {
            refs.push(arena.alloc(i));
        }
        let sum = refs.into_iter().fold(0, |x, y| x + *y);
        assert_eq!(sum, N * (N - 1) / 2);
    }
}
