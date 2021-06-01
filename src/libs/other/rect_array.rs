use std::{
    mem,
    ops::{Index, IndexMut},
    ptr, slice,
};

#[doc(hidden)]
#[macro_export]
macro_rules! __rect_array_row {
    [$elem:expr; $count:expr] => {

    };
    [$(elem:expr),*] => {
        $($elem,)+
    }
}
#[macro_export]
macro_rules! rect_array {
    [$([$row:tt],)*] => {

    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct RectArray<T> {
    buf: Vec<T>,
    n: usize,
    m: usize,
}
impl<T> RectArray<T> {
    pub fn new(m: usize) -> Self {
        Self {
            buf: vec![],
            n: 0,
            m,
        }
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn width(&self) -> usize {
        self.m
    }
    pub fn elems(&self) -> &[T] {
        &self.buf
    }
    pub fn elems_mut(&mut self) -> &mut [T] {
        &mut self.buf
    }
    pub fn get(&self, i: usize) -> Option<&[T]> {
        self.buf.get(self.m * i..self.m * (i + 1))
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut [T]> {
        self.buf.get_mut(self.m * i..self.m * (i + 1))
    }
    pub fn last(&self) -> Option<&[T]> {
        self.n.checked_sub(1).and_then(move |i| self.get(i))
    }
    pub fn last_mut(&mut self) -> Option<&mut [T]> {
        self.n.checked_sub(1).and_then(move |i| self.get_mut(i))
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.n);
        assert!(j < self.n);
        if i != j {
            let ptr = self.buf.as_mut_ptr();
            unsafe {
                ptr::swap_nonoverlapping(ptr.add(self.m * i), ptr.add(self.m * j), self.m);
            }
        }
    }
    pub fn push<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.buf.extend(iter.into_iter().take(self.m));
        self.n += 1;
        assert_eq!(self.buf.len(), self.n * self.m);
    }
    pub fn push_slice(&mut self, row: &[T])
    where
        T: Clone,
    {
        assert_eq!(row.len(), self.width());
        self.buf.extend_from_slice(row);
        self.n += 1;
    }
    pub fn pop(&mut self) -> Option<Row<'_, T>> {
        if self.n == 0 {
            None
        } else {
            unsafe {
                self.n -= 1;
                self.buf.set_len(self.n * self.m);
                let s =
                    slice::from_raw_parts_mut(self.buf.as_mut_ptr().add(self.n * self.m), self.m);
                Some(Row(s.iter_mut()))
            }
        }
    }
    pub fn insert_slice(&mut self, i: usize, row: &[T])
    where
        T: Clone,
    {
        assert!(i <= self.len());
        assert_eq!(row.len(), self.width());
        let add = ((self.n + 1) * self.m).checked_sub(self.buf.capacity());
        if let Some(add) = add {
            self.buf.reserve(add);
        }
        unsafe {
            self.buf.set_len(self.m * i);
            let hole = self.buf.as_mut_ptr().add(self.m * i);
            ptr::copy(hole, hole.add(self.m), self.m * (self.n - i));
            for (i, elem) in row.iter().enumerate() {
                hole.add(i).write(elem.clone());
            }
            self.n += 1;
            self.buf.set_len(self.m * self.n);
        }
    }
}
impl<T> Index<usize> for RectArray<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &[T] {
        assert!(i < self.len());
        unsafe { self.buf.get_unchecked(self.m * i..self.m * (i + 1)) }
    }
}
impl<T> IndexMut<usize> for RectArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut [T] {
        assert!(i < self.len());
        unsafe { self.buf.get_unchecked_mut(self.m * i..self.m * (i + 1)) }
    }
}
pub struct Row<'a, T>(slice::IterMut<'a, T>);
impl<'a, T> Iterator for Row<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| unsafe { (x as *const T).read() })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T> ExactSizeIterator for Row<'a, T> {}
impl<'a, T> Drop for Row<'a, T> {
    fn drop(&mut self) {
        let iter = mem::replace(&mut self.0, [].iter_mut());
        unsafe {
            ptr::drop_in_place(iter.into_slice());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct S<'a, T: Copy>(T, &'a RefCell<Vec<T>>);
    impl<'a, T: Copy> Drop for S<'a, T> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    #[test]
    fn pop() {
        let d = RefCell::new(vec![]);
        let mut a = RectArray::new(3);
        a.push(vec![S(1i32, &d), S(2, &d), S(3, &d)]);
        a.push(vec![S(4, &d), S(5, &d), S(6, &d)]);
        let mut p = a.pop().unwrap();
        assert_eq!(p.next().map(|x| x.0), Some(4));
        drop(p);
        assert_eq!(d.borrow()[..], [4, 5, 6]);
        drop(a);
        assert_eq!(d.borrow()[..], [4, 5, 6, 1, 2, 3]);
    }
}
