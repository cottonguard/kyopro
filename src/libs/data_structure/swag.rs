pub struct Swag<T, F> {
    f: F,
    f_head: Vec<T>,
    tail: Vec<T>,
    f_tail: Option<T>,
}

impl<T: Clone, F: Fn(&T, &T) -> T> Swag<T, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            f_head: Vec::new(),
            tail: Vec::new(),
            f_tail: None,
        }
    }

    pub fn push(&mut self, x: T) {
        self.f_tail = Some(if let Some(y) = self.f_tail.as_ref() {
            (self.f)(y, &x)
        } else {
            x.clone()
        });
        self.tail.push(x);
    }

    pub fn pop(&mut self) {
        if self.f_head.is_empty() {
            for x in self.tail.drain(..).rev() {
                self.f_head.push(if let Some(y) = self.f_head.last() {
                    (self.f)(&x, y)
                } else {
                    x
                });
            }
            self.f_tail = None;
        }
        self.f_head.pop();
    }

    pub fn fold(&self) -> Option<T> {
        match (self.f_head.last(), self.f_tail.as_ref()) {
            (Some(x), Some(y)) => Some((self.f)(x, y)),
            (Some(x), None) => Some(x.clone()),
            (None, Some(y)) => Some(y.clone()),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.f_head.len() + self.tail.len()
    }

    pub fn is_empty(&self) -> bool {
        self.f_head.is_empty() && self.tail.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sliding_minimum_test() {
        let mut swag = Swag::new(|x, y| i32::min(*x, *y));
        assert_eq!(swag.fold(), None);
        swag.push(5);
        assert_eq!(swag.fold(), Some(5));
        swag.push(3);
        assert_eq!(swag.fold(), Some(3));
        assert_eq!(swag.len(), 2);
        swag.pop();
        assert_eq!(swag.fold(), Some(3));
        swag.push(4);
        assert_eq!(swag.fold(), Some(3));
        swag.pop();
        assert_eq!(swag.fold(), Some(4));
        swag.pop();
        assert_eq!(swag.fold(), None);
        swag.pop();
    }
}
