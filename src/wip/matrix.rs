use std::ops;
pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
#[derive(Clone)]
pub struct SquareMat<T> {
    a: Box<[T]>,
    n: usize,
}
impl<T: Zero> SquareMat<T> {
    pub fn zeros(n: usize) -> Self {
        Self {
            a: (0..n * n).map(|_| T::zero()).collect(),
            n,
        }
    }
}
impl<T: Zero + One> SquareMat<T> {
    pub fn id(n: usize) -> Self {
        let mut mat = Self::zeros(n);
        for i in 0..n {
            mat[i][i] = T::one();
        }
        mat
    }
}
impl<T> SquareMat<T> {
    pub fn deg(&self) -> usize {
        self.n
    }
    pub fn transpose(mut self) -> Self {
        for i in 0..self.n {
            for j in i + 1..self.n {
                unsafe {
                    std::ptr::swap(&mut self[i][j], &mut self[j][i]);
                }
            }
        }
        self
    }
    pub fn iter(&self) -> Iter<T> {
        Iter { mat: self, i: 0 }
    }
}
impl<T: Zero + One + ops::AddAssign> SquareMat<T>
where
    for<'a> &'a T: ops::Mul<Output = T>,
{
    pub fn pow(mut self, mut k: u64) -> Self {
        let mut res = Self::id(self.n);
        let mut tmp = Self::zeros(self.n);
        loop {
            if k % 2 == 1 {
                for j in 0..self.n {
                    for i in 0..self.n {
                        tmp[j][i] = T::zero();
                        for k in 0..self.n {
                            tmp[j][i] += &self[i][k] * &res[j][k];
                        }
                    }
                }
                std::mem::swap(&mut res, &mut tmp);
            }
            k /= 2;
            if k == 0 {
                return res.transpose();
            }
            tmp.mul_write(&self, &self);
            std::mem::swap(&mut self, &mut tmp);
        }
    }
}
impl<T: ops::AddAssign + Zero> SquareMat<T>
where
    for<'a> &'a T: ops::Mul<Output = T>,
{
    fn mul_write(&mut self, a: &Self, b: &Self) {
        assert_eq!(self.n, a.n);
        assert_eq!(self.n, b.n);
        for i in 0..self.n {
            for j in 0..self.n {
                self[i][j] = T::zero();
                for k in 0..self.n {
                    self[i][j] += &a[i][k] * &b[k][j];
                }
            }
        }
    }
}
impl<T: Zero + ops::Add<Output = T>> ops::Mul<&[T]> for &SquareMat<T>
where
    for<'a> &'a T: ops::Mul<Output = T>,
{
    type Output = Vec<T>;
    fn mul(self, v: &[T]) -> Self::Output {
        self.iter()
            .map(|r| {
                r.iter()
                    .zip(v.iter())
                    .map(|(x, y)| x * y)
                    .fold(T::zero(), |x, y| x + y)
            })
            .collect()
    }
}
impl<T> ops::Index<usize> for SquareMat<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &[T] {
        assert!(i < self.n);
        unsafe { self.a.get_unchecked(self.n * i..self.n * (i + 1)) }
    }
}
impl<T> ops::IndexMut<usize> for SquareMat<T> {
    fn index_mut(&mut self, i: usize) -> &mut [T] {
        assert!(i < self.n);
        unsafe { self.a.get_unchecked_mut(self.n * i..self.n * (i + 1)) }
    }
}
impl<T: fmt::Debug> fmt::Debug for SquareMat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.n).map(|i| &self[i]))
            .finish()
    }
}
pub struct Iter<'a, T> {
    mat: &'a SquareMat<T>,
    i: usize,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.mat.n {
            let i = self.i;
            self.i += 1;
            Some(&self.mat[i])
        } else {
            None
        }
    }
}