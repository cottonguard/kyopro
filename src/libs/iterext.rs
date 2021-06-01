pub trait IterExt: Iterator {
    fn inner_product<S, P, I, M, A>(mut self, other: I, init: S, mut mul: M, mut add: A) -> S
    where
        Self: Sized,
        I: IntoIterator,
        M: FnMut(Self::Item, I::Item) -> P,
        A: FnMut(S, P) -> S,
    {
        let mut other = other.into_iter();
        let mut res = init;
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => res = add(res, mul(x, y)),
                (None, None) => return res,
                _ => panic!(),
            }
        }
    }
    fn partial_sum<T, F>(self, init: T, f: F) -> PartialSum<Self, F, T>
    where
        Self: Sized,
        F: FnMut(&T, Self::Item) -> T,
    {
        PartialSum {
            inner: self,
            f,
            sum: Some(init),
        }
    }
    fn adjacent_difference<T, U, F>(self, f: F) -> AdjacentDifference<Self, F>
    where
        Self: Sized,
        F: FnMut(T, &T) -> U,
    {
        AdjacentDifference {
            inner: self,
            prev: None,
            f,
        }
    }
    fn cartesian_product<I>(mut self, other: I) -> CartesianProduct<Self, I>
    where
        Self: Sized,
        Self::Item: Clone,
        I: Iterator + Clone,
    {
        CartesianProduct {
            left_val: self.next(),
            left: self,
            right: other.clone(),
            right_orig: other,
        }
    }
}
impl<T: Iterator> IterExt for T {}
pub struct PartialSum<I, F, T> {
    inner: I,
    f: F,
    sum: Option<T>,
}
impl<I, F, T> Iterator for PartialSum<I, F, T>
where
    I: Iterator,
    F: FnMut(&T, I::Item) -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Some(x) = self.inner.next() {
            let new = (self.f)(self.sum.as_ref().unwrap(), x);
            self.sum.replace(new)
        } else {
            self.sum.take()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.sum.is_some() {
            let (l, u) = self.inner.size_hint();
            (l + 1, u.map(|u| u + 1))
        } else {
            (0, Some(0))
        }
    }
}
pub struct AdjacentDifference<I: Iterator, F> {
    inner: I,
    prev: Option<I::Item>,
    f: F,
}
impl<I, F, T> Iterator for AdjacentDifference<I, F>
where
    I: Iterator<Item = T>,
    F: FnMut(I::Item, &I::Item) -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let opt_x = self.prev.take().or_else(|| self.inner.next());
        let opt_y = self.inner.next();
        if let (Some(x), Some(y)) = (opt_x, opt_y.as_ref()) {
            let res = Some((self.f)(x, y));
            self.prev = opt_y;
            res
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, u) = self.inner.size_hint();
        if self.prev.is_none() {
            (l.saturating_sub(1), u.map(|u| u.saturating_sub(1)))
        } else {
            (l, u)
        }
    }
}
pub struct CartesianProduct<I1: Iterator, I2> {
    left: I1,
    left_val: Option<I1::Item>,
    right_orig: I2,
    right: I2,
}
impl<I1: Iterator, I2: Iterator + Clone> Iterator for CartesianProduct<I1, I2>
where
    I1::Item: Clone,
{
    type Item = (I1::Item, I2::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref l) = self.left_val {
            if let Some(r) = self.right.next() {
                Some((l.clone(), r))
            } else {
                self.left_val = self.left.next();
                if let Some(ref l) = self.left_val {
                    self.right = self.right_orig.clone();
                    self.right.next().map(|r| (l.clone(), r))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (left_l, left_u) = self.left.size_hint();
        let (right_l, right_u) = self.right.size_hint();
        let (right_orig_l, right_orig_u) = self.right_orig.size_hint();
        let l = left_l * right_orig_l + right_l;
        let u = match (left_u, right_u, right_orig_u) {
            (Some(left_u), Some(right_u), Some(right_orig_u)) => {
                Some(left_u * right_orig_u + right_u)
            }
            _ => None,
        };
        (l, u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_sum_test() {
        let a = vec![1, 10, 100, 1000, 10000];
        let n = a.len();
        let ps = a.into_iter().partial_sum(0, |x, y| x + y);
        assert_eq!(ps.size_hint(), (n + 1, Some(n + 1)));
        assert!(ps.eq(vec![0, 1, 11, 111, 1111, 11111]));
    }

    #[test]
    fn adjacent_difference_test() {
        let a = vec![3, 1, 4, 1, 5, 9, 2];
        let n = a.len();
        let ad = a.into_iter().adjacent_difference(|x, y| x - y);
        assert_eq!(ad.size_hint(), (n - 1, Some(n - 1)));
        assert!(ad.eq(vec![2, -3, 3, -4, -4, 7]));
    }

    #[test]
    fn inner_product_test() {
        let a = vec![1, 10, 3, 1000, 5];
        let b = vec![1, 2, 100, 4, 10000];
        let ip = a.iter().inner_product(b, 0, |x, y| x * y, |x, y| x + y);
        assert_eq!(ip, 54321);
    }

    #[test]
    fn cartesian_product_test() {
        let a = vec![1, 2, 3];
        let b = "abcd";
        let mut rest = a.len() * b.len();
        let mut cp = a.iter().cartesian_product(b.bytes());
        for x in &a {
            for y in b.bytes() {
                assert_eq!(cp.size_hint(), (rest, Some(rest)));
                assert_eq!(cp.next(), Some((x, y)));
                rest -= 1;
            }
        }
        assert_eq!(cp.size_hint(), (0, Some(0)));
        assert_eq!(cp.next(), None);
    }
}
