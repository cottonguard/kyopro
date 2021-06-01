use std::{
    fmt,
    iter::FromIterator,
    mem::MaybeUninit,
    ops::{Deref, DerefMut, RangeBounds},
    ptr, slice,
};
#[macro_export]
macro_rules! array_vec {
    [$CAP:expr; $($val:expr),* $(,)?] => {{
        let mut a = ArrayVec::<_, $CAP>::new();
        $(a.push($val);)*
        a
    }};
    [$CAP:expr; $val:expr; $n:expr] => {{
        let mut a = ArrayVec::<_, $CAP>::new();
        for _ in 1..$n {
            a.push($val.clone());
        }
        if $n > 0 {
            a.push($val);
        }
        a
    }}
}
pub struct ArrayVec<T, const CAP: usize> {
    len: usize,
    arr: [MaybeUninit<T>; CAP],
}
impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub fn new() -> Self {
        Self {
            len: 0,
            arr: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
    pub const fn is_full(&self) -> bool {
        self.len == CAP
    }
    pub fn as_slice(&self) -> &[T] {
        self
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
    pub fn clear(&mut self) {
        let orig_len = self.len;
        self.len = 0;
        unsafe {
            let s = slice::from_raw_parts_mut(self.arr.as_mut_ptr() as *mut T, orig_len);
            ptr::drop_in_place(s);
        }
    }
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            let orig_len = self.len;
            self.len = len;
            unsafe {
                let s = slice::from_raw_parts_mut(
                    self.arr.as_mut_ptr().add(len) as *mut T,
                    len - orig_len,
                );
                ptr::drop_in_place(s);
            }
        }
    }
    pub fn push(&mut self, val: T) -> Option<T> {
        if let Some(a) = self.arr.get_mut(self.len) {
            *a = MaybeUninit::new(val);
            self.len += 1;
            None
        } else {
            Some(val)
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            unsafe { Some(self.arr.as_ptr().add(self.len).read().assume_init()) }
        }
    }
    pub fn insert(&mut self, i: usize, val: T) {
        assert!(i <= self.len(), "index ({}) > len ({})", i, self.len());
        assert!(self.is_full(), "len = capacity ({})", CAP);
        unsafe {
            ptr::copy(
                self.arr.as_ptr().add(i),
                self.arr.as_mut_ptr().add(i + 1),
                self.len - i,
            );
        }
        self.arr[i] = MaybeUninit::new(val);
        self.len += 1;
    }
    pub fn remove(&mut self, i: usize) -> T {
        assert!(i < self.len());
        unsafe {
            let res = self.arr.as_ptr().add(i).read().assume_init();
            ptr::copy(
                self.arr.as_ptr().add(i + 1),
                self.arr.as_mut_ptr().add(i),
                self.len - i - 1,
            );
            self.len -= 1;
            res
        }
    }
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.dedup_by(|x, y| x == y)
    }
    pub fn dedup_by(&mut self, mut f: impl FnMut(&T, &T) -> bool) {
        let orig_len = self.len;
        self.len = 0;
        let mut new_len = 0;
        let mut l = 0;
        for i in 1..orig_len {
            unsafe {
                if f(&*self.arr[i - 1].as_ptr(), &*self.arr[i].as_ptr()) {
                    ptr::drop_in_place(&mut *self.arr[i].as_mut_ptr());
                    let cnt = i - l;
                    ptr::copy(
                        self.arr.as_ptr().add(l),
                        self.arr.as_mut_ptr().add(new_len),
                        cnt,
                    );
                    new_len += cnt;
                    l = i + 1;
                }
            }
        }
        self.len = new_len;
    }
    pub fn dedup_by_key<U: PartialEq>(&mut self, mut f: impl FnMut(&T) -> U) {
        self.dedup_by(|x, y| f(x) == f(y))
    }
    pub fn retain(&mut self, mut f: impl FnMut(&T) -> bool) {
        let orig_len = self.len;
        self.len = 0;
        let mut new_len = 0;
        let mut retain_begin = 0;
        for i in 0..orig_len {
            unsafe {
                if !f(&*self.arr[i].as_ptr()) {
                    ptr::drop_in_place(&mut *self.arr[i].as_mut_ptr());
                    let cnt = i - retain_begin;
                    ptr::copy(
                        self.arr.as_ptr().add(retain_begin),
                        self.arr.as_mut_ptr().add(new_len),
                        cnt,
                    );
                    new_len += cnt;
                    retain_begin = i + 1;
                }
            }
        }
        self.len = new_len;
    }
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T, CAP> {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(i) => *i,
            Excluded(i) => i.saturating_sub(1),
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(i) => i + 1,
            Excluded(i) => *i,
            Unbounded => self.len,
        };
        assert!(start <= end);
        assert!(end <= self.len());
        let len = self.len;
        self.len = start;
        Drain {
            av: self,
            pos: start,
            end,
            len,
        }
    }
}
impl<T, const CAP: usize> Drop for ArrayVec<T, CAP> {
    fn drop(&mut self) {
        self.clear();
    }
}
impl<T, const CAP: usize> Deref for ArrayVec<T, CAP> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.arr.as_ptr() as *const T, self.len) }
    }
}
impl<T, const CAP: usize> DerefMut for ArrayVec<T, CAP> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.arr.as_mut_ptr() as *mut T, self.len) }
    }
}
impl<T: Clone, const CAP: usize> Clone for ArrayVec<T, CAP> {
    fn clone(&self) -> Self {
        self.as_slice().into()
    }
}
impl<T: PartialEq, const CAP: usize> PartialEq for ArrayVec<T, CAP> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
impl<T: Eq, const CAP: usize> Eq for ArrayVec<T, CAP> {}
impl<T: PartialOrd, const CAP: usize> PartialOrd for ArrayVec<T, CAP> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
impl<T: Ord, const CAP: usize> Ord for ArrayVec<T, CAP> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(&other.as_slice())
    }
}
impl<T: std::hash::Hash, const CAP: usize> std::hash::Hash for ArrayVec<T, CAP> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}
impl<T: fmt::Debug, const CAP: usize> fmt::Debug for ArrayVec<T, CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}
impl<T, const CAP: usize> IntoIterator for ArrayVec<T, CAP> {
    type Item = T;
    type IntoIter = IntoIter<T, CAP>;
    fn into_iter(mut self) -> IntoIter<T, CAP> {
        let len = self.len;
        self.len = 0;
        IntoIter {
            av: self,
            pos: 0,
            len,
        }
    }
}
impl<T, const CAP: usize> FromIterator<T> for ArrayVec<T, CAP> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut a = Self::new();
        a.extend(iter);
        a
    }
}
impl<T, const CAP: usize> Extend<T> for ArrayVec<T, CAP> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        if self.is_full() {
            return;
        }
        for val in iter {
            self.push(val);
            if self.is_full() {
                return;
            }
        }
    }
}
impl<T: Clone, const CAP: usize> From<&[T]> for ArrayVec<T, CAP> {
    fn from(slice: &[T]) -> Self {
        slice.iter().cloned().collect()
    }
}
pub struct IntoIter<T, const CAP: usize> {
    av: ArrayVec<T, CAP>,
    pos: usize,
    len: usize,
}
impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.pos < self.len {
            let res = unsafe { self.av.arr[self.pos].as_ptr().read() };
            self.pos += 1;
            Some(res)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len - self.pos;
        (size, Some(size))
    }
}
impl<T, const CAP: usize> DoubleEndedIterator for IntoIter<T, CAP> {
    fn next_back(&mut self) -> Option<T> {
        if self.pos < self.len {
            self.len -= 1;
            let res = unsafe { self.av.arr[self.len].as_ptr().read() };
            Some(res)
        } else {
            None
        }
    }
}
impl<T, const CAP: usize> ExactSizeIterator for IntoIter<T, CAP> {}
impl<T, const CAP: usize> Drop for IntoIter<T, CAP> {
    fn drop(&mut self) {
        unsafe {
            let s =
                slice::from_raw_parts_mut(self.av.arr.as_mut_ptr() as *mut T, self.len - self.pos);
            ptr::drop_in_place(s);
        }
    }
}
pub struct Drain<'a, T, const CAP: usize> {
    av: &'a mut ArrayVec<T, CAP>,
    pos: usize,
    end: usize,
    len: usize,
}
impl<'a, T, const CAP: usize> Iterator for Drain<'a, T, CAP> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.pos < self.end {
            let res = unsafe { self.av.arr[self.pos].as_ptr().read() };
            self.pos += 1;
            Some(res)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end - self.pos;
        (size, Some(size))
    }
}
impl<'a, T, const CAP: usize> DoubleEndedIterator for Drain<'a, T, CAP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
impl<'a, T, const CAP: usize> ExactSizeIterator for Drain<'a, T, CAP> {}
impl<'a, T, const CAP: usize> Drop for Drain<'a, T, CAP> {
    fn drop(&mut self) {
        let start = self.av.len;
        let rem = self.end - self.pos;
        unsafe {
            let s =
                slice::from_raw_parts_mut(self.av.arr.as_mut_ptr().add(self.pos) as *mut T, rem);
            ptr::drop_in_place(s);
            ptr::copy(
                self.av.arr.as_ptr().add(self.end),
                self.av.arr.as_mut_ptr().add(start),
                self.len - self.end,
            );
        }
        self.av.len = self.len - (self.end - start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct S<'a>(i32, &'a RefCell<Vec<i32>>);
    impl<'a> Drop for S<'a> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    #[test]
    fn array_vec_test() {
        let mut a = ArrayVec::<i32, 3>::new();
        assert_eq!(a.push(10), None);
        assert_eq!(*a, [10]);
        assert_eq!(a.push(20), None);
        assert_eq!(*a, [10, 20]);
        assert_eq!(a.push(30), None);
        assert_eq!(*a, [10, 20, 30]);
        assert_eq!(a.push(40), Some(40));
        assert_eq!(*a, [10, 20, 30]);
    }

    #[test]
    fn drop_test() {
        let dropped = RefCell::new(vec![]);
        let a: ArrayVec<_, 10> = (1..=5).map(|x| S(x, &dropped)).collect();
        drop(a);
        assert_eq!(dropped.borrow()[..], [1, 2, 3, 4, 5]);
    }

    #[test]
    fn insert() {
        /*
        let mut a: ArrayVec<_, 8> = (1..=5).collect();
        assert_eq!(a.insert(2, 10), None);
        assert_eq!(*a, [1, 2, 10, 3, 4, 5]);
        assert_eq!(a.insert(0, 20), None);
        assert_eq!(*a, [20, 1, 2, 10, 3, 4, 5]);
        assert_eq!(a.insert(7, 30), None);
        assert_eq!(*a, [20, 1, 2, 10, 3, 4, 5, 30]);
        assert_eq!(a.insert(2, 40), Some(30));
        assert_eq!(*a, [20, 1, 40, 2, 10, 3, 4, 5]);
        */
    }

    #[test]
    fn dedup() {
        let mut a = array_vec![20; 1, 1, 2, 3, 3, 3, 4, 5, 5, 5];
        a.dedup();
        assert_eq!(a.as_slice(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn retain() {
        let dropped = RefCell::new(vec![]);
        let b = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut a: ArrayVec<_, 10> = b.into_iter().map(|x| S(x, &dropped)).collect();
        a.retain(|S(x, _)| *x <= 2);
        assert_eq!(dropped.borrow()[..], [3, 4, 5, 9, 6, 5]);
        assert!(a.iter().map(|S(x, _)| *x).eq(vec![1, 1, 2]));
    }

    #[test]
    fn into_iter() {
        let a: ArrayVec<_, 10> = (0..5).collect();
        let mut it = a.into_iter();
        assert_eq!(it.next_back(), Some(4));
        assert!(it.take(3).eq(vec![0, 1, 2]));
    }

    #[test]
    fn drain() {
        let dropped = RefCell::new(vec![]);
        let mut a: ArrayVec<_, 13> = (0..10).map(|x| S(x, &dropped)).collect();
        let mut d = a.drain(3..=7);
        assert_eq!(d.next().map(|S(x, _)| x), Some(3));
        assert_eq!(d.next().map(|S(x, _)| x), Some(4));
        assert_eq!(d.next().map(|S(x, _)| x), Some(5));
        assert_eq!(dropped.borrow()[..], [3, 4, 5]);
        drop(d);
        assert_eq!(dropped.borrow()[..], [3, 4, 5, 6, 7]);
        assert!(a.iter().map(|S(x, _)| *x).eq(vec![0, 1, 2, 8, 9]));

        let mut a: ArrayVec<_, 13> = (0..2).collect();
        let mut d = a.drain(..);
        assert_eq!(d.next(), Some(0));
        assert_eq!(d.next(), Some(1));
        assert_eq!(d.next(), None);
        assert_eq!(d.next(), None);
    }
}
