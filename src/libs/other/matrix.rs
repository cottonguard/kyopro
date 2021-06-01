use std::{
    fmt,
    mem::{self, MaybeUninit},
    ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub},
    ptr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    data: Box<[T]>,
    n: usize,
    m: usize,
}
impl<T> Matrix<T> {
    pub fn repeat(n: usize, m: usize, x: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![x; n * m].into(),
            n,
            m,
        }
    }
    pub fn repeat_with(n: usize, m: usize, mut f: impl FnMut() -> T) -> Self {
        Self::from_elems(n, m, (0..n * m).map(|_| f()))
    }
    pub fn from_elems(n: usize, m: usize, iter: impl IntoIterator<Item = T>) -> Self {
        let data: Box<[T]> = iter.into_iter().take(n * m).collect();
        assert_eq!(data.len(), n * m);
        Self { data, n, m }
    }
    pub fn uninit(n: usize, m: usize) -> Matrix<MaybeUninit<T>> {
        let mut data = Vec::with_capacity(n * m);
        unsafe {
            data.set_len(n * m);
        }
        Matrix {
            data: data.into(),
            n,
            m,
        }
    }
    pub fn zeros(n: usize, m: usize) -> Self
    where
        T: Zero,
    {
        Self::repeat_with(n, m, T::zero)
    }
    pub fn id(n: usize) -> Self
    where
        T: Zero + One,
    {
        let mut res = Self::zeros(n, n);
        for (i, row) in res.iter_mut().enumerate() {
            row[i] = T::one();
        }
        res
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn dim(&self) -> (usize, usize) {
        (self.n, self.m)
    }
    pub fn is_square(&self) -> bool {
        self.n == self.m
    }
    pub fn get(&self, i: usize) -> Option<&[T]> {
        self.data.get(self.m * i..self.m * (i + 1))
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut [T]> {
        self.data.get_mut(self.m * i..self.m * (i + 1))
    }
    pub fn elems(&self) -> &[T] {
        &self.data
    }
    pub fn elems_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { mat: self, i: 0 }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            data: &mut self.data,
            m: self.m,
        }
    }
    pub fn swap(&mut self, i1: usize, i2: usize) {
        if i1 != i2 {
            for j in 0..self.m {
                self.data.swap(self.m * i1 + j, self.m * i2 + j);
            }
        }
    }
    pub fn transpose(mut self) -> Self {
        if self.n == self.m {
            let n = self.n;
            for i in 0..n {
                for j in i + 1..n {
                    self.data.swap(n * i + j, n * j + i);
                }
            }
            self
        } else {
            let mut new_data: Vec<T> = Vec::with_capacity(self.n * self.m);
            let mut orig_data: Vec<T> = self.data.into();
            let orig_ptr = orig_data.as_ptr();
            let new_ptr = new_data.as_mut_ptr();
            unsafe {
                orig_data.set_len(0);
                for i in 0..self.n {
                    for j in 0..self.m {
                        ptr::copy_nonoverlapping(
                            orig_ptr.add(self.m * i + j),
                            new_ptr.add(self.n * j + i),
                            1,
                        );
                    }
                }
                new_data.set_len(self.n * self.m);
            }
            Self {
                data: new_data.into(),
                n: self.m,
                m: self.n,
            }
        }
    }
    pub fn pow(mut self, mut exp: u64) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + One + Copy,
    {
        assert!(self.is_square());
        let mut self_temp = Self::zeros(self.n, self.n);
        let mut res = Self::id(self.n);
        let mut res_temp = Self::zeros(self.n, self.n);
        while exp > 0 {
            if exp % 2 == 1 {
                res_temp.mul_write(&self, &res);
                mem::swap(&mut res.data, &mut res_temp.data);
            }
            exp /= 2;
            self_temp.mul_write(&self, &self);
            mem::swap(&mut self.data, &mut self_temp.data);
        }
        res
    }
    pub fn gaussian_elimination(&mut self) -> usize
    where
        T: Eq + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero + Copy,
    {
        let mut pj = 0;
        for pi in 0..self.n {
            for i in pi..self.n {
                if self[i][pj] != T::zero() {
                    self.swap(i, pi);
                    for i in pi + 1..self.n {
                        if self[i][pj] == T::zero() {
                            continue;
                        }
                        let x = self[i][pj] / self[pi][pj];
                        self[i][pj] = T::zero();
                        for j in pj + 1..self.m {
                            self[i][j] = self[i][j] - x * self[pi][j];
                        }
                    }
                    pj += 1;
                    if pj >= self.n {
                        return self.n;
                    }
                    break;
                }
            }
        }
        pj
    }
    fn mul_write(&mut self, lhs: &Self, rhs: &Self)
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
    {
        assert_eq!(self.n, lhs.n);
        assert_eq!(self.m, rhs.m);
        assert_eq!(lhs.m, rhs.n);
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = T::zero();
                for k in 0..lhs.m {
                    self[i][j] = self[i][j] + lhs[i][k] * rhs[k][j];
                }
            }
        }
    }
    fn assert_dim(&self, other: &Self) {
        assert_eq!(self.dim(), other.dim());
    }
}
impl<T> Matrix<MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> Matrix<T> {
        let data = Box::from_raw(Box::into_raw(self.data) as *mut [T]);
        Matrix {
            data,
            n: self.n,
            m: self.m,
        }
    }
}
impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.n);
        &self.data[self.m * i..self.m * (i + 1)]
    }
}
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.n);
        &mut self.data[self.m * i..self.m * (i + 1)]
    }
}
impl<T: Neg<Output = T> + Copy> Neg for Matrix<T> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for e in self.data.iter_mut() {
            *e = -*e;
        }
        self
    }
}
impl<'a, T: Mul<Output = T> + Add<Output = T> + Zero + Copy> Mul for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::zeros(self.n, rhs.m);
        res.mul_write(self, rhs);
        res
    }
}
impl<'a, T: Mul<Output = T> + Add<Output = T> + Zero + Copy> MulAssign<&'a Matrix<T>>
    for Matrix<T>
{
    fn mul_assign(&mut self, rhs: &'a Self) {
        assert_eq!(self.m, rhs.m);
        *self = &*self * rhs;
    }
}
impl<'a, T: Mul<Output = T> + Add<Output = T> + Zero + Copy> Mul<&'a [T]> for &'a Matrix<T> {
    type Output = Vec<T>;
    fn mul(self, rhs: &'a [T]) -> Self::Output {
        assert_eq!(self.m, rhs.len());
        let mut res = Vec::with_capacity(self.n);
        let it = self.iter().map(|row| {
            row.iter()
                .zip(rhs.iter())
                .map(|(x, y)| *x * *y)
                .fold(T::zero(), |s, p| s + p)
        });
        res.extend(it);
        res
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Matrix<T> {
    type Output = Self;
    fn mul(mut self, x: T) -> Self {
        self *= x;
        self
    }
}
impl<T: Mul<Output = T> + Copy> MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, x: T) {
        for e in self.elems_mut() {
            *e = *e * x;
        }
    }
}
macro_rules! op {
    ($($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident);+) => {$(
        impl<'a, T: std::ops::$Op<Output = T> + Copy> std::ops::$Op for &'a Matrix<T> {
            type Output = Matrix<T>;
            fn $op(self, rhs: Self) -> Matrix<T> {
                self.assert_dim(rhs);
                let iter = self.data.iter().zip(rhs.data.iter()).map(|(x, y)| (*x).$op(*y));
                Matrix::from_elems(self.n, self.m, iter)
            }
        }
        impl<T: std::ops::$Op<Output = T> + Copy> std::ops::$Op for Matrix<T> {
            type Output = Self;
            fn $op(self, rhs: Self) -> Self {
                (&self).$op(&rhs)
            }
        }
        impl<'a, T: std::ops::$OpAssign + Copy> std::ops::$Op<&'a Matrix<T>> for Matrix<T> {
            type Output = Self;
            fn $op(mut self, rhs: &'a Self) -> Self {
                use std::ops::$OpAssign;
                self.$op_assign(rhs);
                self
            }
        }
        impl<'a, T: std::ops::$Op<Output = T> + Copy> std::ops::$Op<Matrix<T>> for &'a Matrix<T> {
            type Output = Matrix<T>;
            fn $op(self, rhs: Matrix<T>) -> Matrix<T> {
                self.$op(&rhs)
            }
        }
        impl<T: std::ops::$OpAssign + Copy> std::ops::$OpAssign for Matrix<T> {
            fn $op_assign(&mut self, rhs: Matrix<T>) {
                self.$op_assign(&rhs)
            }
        }
        impl<'a, T: std::ops::$OpAssign + Copy> std::ops::$OpAssign<&'a Matrix<T>> for Matrix<T> {
            fn $op_assign(&mut self, rhs: &'a Matrix<T>) {
                self.assert_dim(rhs);
                for (x, y) in self.data.iter_mut().zip(rhs.data.iter()) {
                    x.$op_assign(*y);
                }
            }
        }
    )+};
}
op!(
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    BitXor, bitxor, BitXorAssign, bitxor_assign
);
impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T: 'a> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut [T];
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
pub struct Iter<'a, T> {
    mat: &'a Matrix<T>,
    i: usize,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        self.mat.get(i)
    }
}
pub struct IterMut<'a, T> {
    data: &'a mut [T],
    m: usize,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let temp = mem::replace(&mut self.data, &mut []);
            let (row, rest) = temp.split_at_mut(self.m);
            self.data = rest;
            Some(row)
        }
    }
}
#[macro_export]
macro_rules! matrix (
    [$([$($elem:expr),*]),*] => {{
        let units = [$([$(matrix!(unit $elem)),*]),*];
        let n = units.len();
        let m = units.first().map(|row| row.len()).unwrap_or(0);
        let data = vec![$($($elem),*),*];
        Matrix {
            data: data.into(), n, m
        }
    }};
    (unit $elem:expr) => {()}
);
pub trait Zero {
    fn zero() -> Self;
}
impl<T: From<bool>> Zero for T {
    fn zero() -> Self {
        false.into()
    }
}
pub trait One {
    fn one() -> Self;
}
impl<T: From<bool>> One for T {
    fn one() -> Self {
        true.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::random::*;

    use super::*;

    #[test]
    fn matrix_macro() {
        let a = matrix![[1, 2, 3], [4, 5, 6]];
        assert_eq!(a.dim(), (2, 3));
        assert_eq!(a[0], [1, 2, 3]);
        assert_eq!(a[1], [4, 5, 6]);
    }

    #[test]
    fn matrix_create() {
        assert_eq!(
            Matrix::<i32>::id(3),
            matrix![[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        );
    }

    #[test]
    fn matrix_add() {
        let a = matrix![[1, 2, 3], [3, 4, 5]];
        let b = matrix![[10, 20, 30], [30, 40, 50]];
        let res = matrix![[11, 22, 33], [33, 44, 55]];
        assert_eq!(&a + &b, res);
        assert_eq!(&a + b.clone(), res);
        assert_eq!(a.clone() + &b, res);
        assert_eq!(a + b, res);
    }

    #[test]
    fn matrix_mul() {
        let mut rand = Pcg::seed_from_u64(1818);
        let n = 10;
        let h = 15;
        let m = 20;
        let a = Matrix::from_elems(n, h, std::iter::repeat_with(|| rand.next_u32() & 0xfff));
        let b = Matrix::from_elems(h, m, std::iter::repeat_with(|| rand.next_u32() & 0xfff));
        let res = &a * &b;
        assert_eq!(res.dim(), (n, m));
        let tb = b.transpose();
        for i in 0..n {
            for j in 0..m {
                let p: u32 = a[i].iter().zip(tb[j].iter()).map(|(x, y)| x * y).sum();
                assert_eq!(res[i][j], p);
            }
        }
    }

    #[test]
    fn matrix_pow() {
        let a = matrix![[1, 2], [3, 4]];
        let p = a.pow(5);
        assert_eq!(p, matrix![[1069, 1558], [2337, 3406]]);
    }

    #[test]
    fn matrix_gaussian_elimination() {}
}
