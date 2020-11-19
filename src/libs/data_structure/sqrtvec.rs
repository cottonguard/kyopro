use std::slice::SliceIndex;
const SIZE: usize = 256;
#[derive(Clone)]
enum Block<T> {
    Covered(T),
    Data([T; SIZE]),
}
impl<T: Copy> Block<T> {
    fn expand(&mut self) {
        if let Self::Covered(v) = self {
            *self = Self::Data([*v; SIZE]);
        }
    }
    fn fill<I: SliceIndex<[T], Output = [T]>>(&mut self, range: I, v: T) {
        self.expand();
        if let Self::Data(a) = self {
            for x in &mut a[range] {
                *x = v;
            }
        }
    }
}
struct SqrtVec<T> {
    blocks: Vec<Block<T>>,
    len: usize,
}
impl<T: Copy> SqrtVec<T> {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            len: 0,
        }
    }
    pub fn resize(&mut self, n: usize, v: T) {
        self.blocks.resize((n + SIZE - 1) / SIZE, Block::Covered(v));
        self.len = n;
    }
    pub fn fill(&mut self, range: std::ops::Range<usize>, v: T) {
        let ldiv = range.start / SIZE;
        let lrem = range.start % SIZE;
        let rdiv = range.end / SIZE;
        let rrem = range.end % SIZE;
        if ldiv == rdiv {
            self.blocks[ldiv].fill(lrem..rrem, v);
        } else {
            self.blocks[ldiv].fill(lrem.., v);
            for b in &mut self.blocks[ldiv + 1..rdiv] {
                *b = Block::Covered(v);
            }
            self.blocks.get_mut(rdiv).map(|b| b.fill(..rrem, v));
        }
    }
    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            self.blocks.get(i / SIZE).map(|b| match b {
                Block::Covered(v) => v,
                Block::Data(a) => &a[i % SIZE],
            })
        } else {
            None
        }
    }
}
impl<T: Copy> std::ops::Index<usize> for SqrtVec<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        self.get(i).unwrap()
    }
}