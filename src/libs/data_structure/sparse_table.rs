pub struct SparseTable<T>(Box<[Box<[T]>]>);
impl<T: Ord + Clone> SparseTable<T> {
    pub fn new<A: Into<Box<[T]>>>(a: A) -> Self {
        let mut a = a.into();
        let mut tab = Vec::new();
        let mut width = 1;
        while width < a.len() {
            let mut b = Vec::with_capacity(a.len() - width);
            for i in width..a.len() {
                b.push((&a[i - width]).min(&a[i]).clone());
            }
            tab.push(a);
            a = b.into();
            width *= 2;
        }
        tab.push(a);
        Self(tab.into())
    }
    pub fn len(&self) -> usize {
        self.0.first().map(|a| a.len()).unwrap_or(0)
    }
    pub fn min(&self, l: usize, r: usize) -> &T {
        assert!(l < r);
        assert!(r <= self.len());
        let k = 8 * std::mem::size_of::<usize>() - (r - l).leading_zeros() as usize - 1;
        (&self.0[k][l]).min(&self.0[k][r - (1 << k)])
    }
}
impl<T: Ord + Clone> std::iter::FromIterator<T> for SparseTable<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect::<Box<[T]>>())
    }
}
impl<T> std::ops::Deref for SparseTable<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.0.first().unwrap()
    }
}
