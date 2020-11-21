pub struct Kmp<T> {
    pat: Vec<T>,
    tab: Vec<usize>,
}

impl<T: PartialEq> Kmp<T> {
    pub fn new(pat: impl Into<Vec<T>>) -> Self {
        let pat = pat.into();
        let mut tab = vec![0; pat.len()];
        let mut j = 0;
        for (i, p) in pat.iter().enumerate().skip(1) {
            while j > 0 && p != &pat[j] {
                j = tab[j - 1];
            }
            j = if p == &pat[j] { j + 1 } else { 0 };
            tab[i] = j;
        }
        Self { pat, tab }
    }

    pub fn len(&self) -> usize {
        self.pat.len()
    }

    pub fn search<'a>(&'a self, s: &'a [T]) -> KmpSearcher<'a, T> {
        KmpSearcher {
            pat: self,
            s,
            pos: 0,
        }
    }

    fn search_next(&self, s: &[T], mut pos: usize) -> usize {
        let mut i = if pos == 0 {
            0
        } else {
            self.tab[self.len() - 1]
        };
        while pos < s.len() {
            while i > 0 && self.pat[i] != s[pos] {
                i = self.tab[i - 1];
            }
            i = if self.pat[i] == s[pos] { i + 1 } else { 0 };
            pos += 1;
            if i == self.len() {
                return pos;
            }
        }
        s.len()
    }
}

pub struct KmpSearcher<'a, T> {
    pat: &'a Kmp<T>,
    s: &'a [T],
    pos: usize,
}

impl<'a, T: PartialEq> Iterator for KmpSearcher<'a, T> {
    type Item = usize;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.pos = self.pat.search_next(self.s, self.pos);
        if self.pos < self.s.len() {
            Some(self.pos - self.pat.len())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let kmp = Kmp::new("abcab");
        let mut s = kmp.search(b"babcabcabacabcaba");
        assert_eq!(s.next(), Some(1));
        assert_eq!(s.next(), Some(4));
        assert_eq!(s.next(), Some(11));
        assert_eq!(s.next(), None);
    }
}
