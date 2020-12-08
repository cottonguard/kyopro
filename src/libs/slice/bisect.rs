pub fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize {
    bisect(a, |v| v >= x)
}
pub fn upper_bound<T: Ord>(a: &[T], x: &T) -> usize {
    bisect(a, |v| v > x)
}
fn bisect<T: Ord, F: Fn(&T) -> bool>(a: &[T], cond: F) -> usize {
    let mut l = -1;
    let mut r = a.len() as isize;
    while r - l > 1 {
        let m = (l + r) / 2;
        if cond(&a[m as usize]) {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn lower_bound() {
        let a = [1, 1, 2, 2, 2, 3, 4, 10, 10, 11];
        assert_eq!(super::lower_bound(&a, &2), 2);
        assert_eq!(super::lower_bound(&a, &1), 0);
        assert_eq!(super::lower_bound(&a, &7), 7);
        assert_eq!(super::lower_bound(&a, &100), a.len());
    }
    #[test]
    fn upper_bound() {
        let a = [1, 1, 2, 2, 2, 3, 4, 10, 10, 11];
        assert_eq!(super::upper_bound(&a, &2), 5);
        assert_eq!(super::upper_bound(&a, &1), 2);
        assert_eq!(super::upper_bound(&a, &7), 7);
        assert_eq!(super::upper_bound(&a, &100), a.len());
    }
}
