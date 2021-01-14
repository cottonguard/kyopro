pub struct Trie {
    bits: Vec<u128>,
    links: Vec<Vec<usize>>,
    in_dict: Vec<bool>,
}
impl Trie {
    pub fn new() -> Self {
        Self {
            bits: vec![0],
            links: vec![vec![]],
            in_dict: vec![false],
        }
    }
    pub fn insert(&mut self, s: &[u8]) {
        let mut u = 0;
        for &c in s {
            let p = (self.bits[u] & (1 << c) - 1).count_ones() as usize;
            u = if self.bits[u] & 1 << c != 0 {
                self.links[u][p]
            } else {
                self.bits[u] |= 1 << c;
                let v = self.links.len();
                self.links[u].insert(p, v);
                self.bits.push(0);
                self.links.push(vec![]);
                self.in_dict.push(false);
                v
            };
        }
        self.in_dict[u] = true;
    }
    pub fn find(&self, s: &[u8]) -> Option<usize> {
        let mut u = 0;
        for &c in s {
            let p = (self.bits[u] & (1 << c) - 1).count_ones() as usize;
            u = if self.bits[u] & 1 << c != 0 {
                self.links[u][p]
            } else {
                return None;
            }
        }
        if self.in_dict[u] {
            Some(u)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie() {
        let mut trie = Trie::new();
        trie.insert(b"asdf");
        trie.insert(b"bsdf");
        trie.insert(b"asas");
        trie.insert(b"asap");
        trie.insert(b"as");
        trie.insert(b"asdfg");
        assert!(trie.find(b"asdf").is_some());
        assert!(trie.find(b"bsdf").is_some());
        assert!(trie.find(b"asas").is_some());
        assert!(trie.find(b"asap").is_some());
        assert!(trie.find(b"as").is_some());
        assert!(trie.find(b"asdfg").is_some());
        assert!(trie.find(b"a").is_none());
        assert!(trie.find(b"asdq").is_none());
        assert!(trie.find(b"csdf").is_none());
        assert!(trie.find(b"asd").is_none());
        assert!(trie.find(b"").is_none());
    }
}
