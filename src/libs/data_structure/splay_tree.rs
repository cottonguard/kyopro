pub struct SplayTreeMap<K, V> {
    node: Option<Box<Node<K, V>>>,
}
struct Node<K, V> {
    key: K,
    value: V,
    left: SplayTreeMap<K, V>,
    right: SplayTreeMap<K, V>,
}
impl<K: Ord, V> SplayTreeMap<K, V> {
    pub fn new() -> Self {
        Self { node: None }
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.splay(key);
        self.node
            .as_ref()
            .and_then(|s| if &s.key == key { Some(&s.value) } else { None })
    }
    pub fn insert(&mut self, key: K, value: V) {
        use std::cmp::Ordering::*;
        self.splay(&key);
        let (left, right) = if let Some(s) = self.node.as_mut() {
            match key.cmp(&s.key) {
                Equal => {
                    s.value = value;
                    return;
                }
                Less => (s.left.take(), self.take()),
                Greater => {
                    let r = s.right.take();
                    (self.take(), r)
                }
            }
        } else {
            (Self::new(), Self::new())
        };
        self.node = Some(Box::new(Node {
            key,
            value,
            left,
            right,
        }));
    }
    fn splay(&mut self, key: &K) {
        use std::cmp::Ordering::*;
        if let Some(s) = self.node.as_mut() {
            match key.cmp(&s.key) {
                Less => {
                    if let Some(l) = s.left.node.as_mut() {
                        match key.cmp(&l.key) {
                            Less => {
                                l.left.splay(key);
                                self.rot_right();
                            }
                            Greater => {
                                l.right.splay(key);
                                s.left.rot_left();
                            }
                            _ => {}
                        }
                        self.rot_right();
                    }
                }
                Greater => {
                    if let Some(r) = s.right.node.as_mut() {
                        match key.cmp(&r.key) {
                            Less => {
                                r.left.splay(key);
                                s.right.rot_right();
                            }
                            Greater => {
                                r.right.splay(key);
                                self.rot_left();
                            }
                            _ => {}
                        }
                        self.rot_left();
                    }
                }
                _ => {}
            }
        }
    }
    fn take(&mut self) -> Self {
        Self {
            node: self.node.take(),
        }
    }
    fn replace(&mut self, node: Box<Node<K, V>>) -> Self {
        Self {
            node: self.node.replace(node),
        }
    }
    fn rot_left(&mut self) {
        if let Some(s) = self.node.as_mut() {
            if let Some(mut r) = s.right.node.take() {
                std::mem::swap(s, &mut r);
                r.right = s.left.take();
                s.left.replace(r);
            }
        }
    }
    fn rot_right(&mut self) {
        if let Some(s) = self.node.as_mut() {
            if let Some(mut l) = s.left.node.take() {
                std::mem::swap(s, &mut l);
                l.left = s.right.take();
                s.right.replace(l);
            }
        }
    }
}

use std::fmt;
impl<K: fmt::Display, V> fmt::Display for SplayTreeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(s) = self.node.as_ref() {
            write!(f, "(")?;
            s.left.fmt(f)?;
            write!(f, " {} ", s.key)?;
            s.right.fmt(f)?;
            write!(f, ")")?;
        } else {
            write!(f, ".")?;
        }
        Ok(())
    }
}
