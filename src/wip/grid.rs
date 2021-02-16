#[derive(Clone)]
pub struct Grid {
    width: usize,
    len: usize,
    data: Vec<u8>,
}
impl Grid {
    pub fn new(width: usize) -> Self {
        Self {
            len: 0,
            width,
            data: vec![]
        }
    }
    pub fn with_capacity(width: usize, cap: usize) -> Self {
        Self {
            len: 0,
            width,
            data: Vec::with_capacity(width * cap)
        }
    }
    pub fn filled(width: usize, len: usize, c: u8) -> Self {
        Self {
            len, width, data: vec![c; len * width],
        }
    }
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(self.width * additional);
    }
    pub fn get(&self, i: usize) -> Option<&[u8]> {
        self.data.get(self.width * i..self.width * (i + 1))
    }
    pub fn push<T: AsRef<[u8]>>(&mut self, row: T) {
        let row = row.as_ref();
        assert_eq!(row.len(), self.width);
        self.data.extend_from_slice(row);
        self.len += 1;
    }
    pub fn iter(&self) -> Iter {
        Iter {
            grid: self,
            i: 0
        }
    }
}
impl std::ops::Index<usize> for Grid {
    type Output = [u8];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.width * i..self.width * (i + 1)]
    }
}
impl std::ops::IndexMut<usize> for Grid {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[self.width * i..self.width * (i + 1)]
    }
}
impl<A: AsRef<[u8]>> Extend<A> for Grid {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for row in iter {
            self.push(row);
        }
    }
}
impl<'a> IntoIterator for &'a Grid {
    type Item = &'a [u8];
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub struct Iter<'a> {
    grid: &'a Grid,
    i: usize,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.grid.get(self.i);
        self.i += 1;
        res
    }
}

