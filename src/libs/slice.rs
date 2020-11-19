pub fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize {
    let mut l = -1;
    let mut r = a.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if &a[m as usize] >= x {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}

pub fn next_permutation<T: Ord>(p: &mut [T]) -> bool {
    for i in (0..p.len() - 1).rev() {
        if p[i] < p[i + 1] {
            for j in (0..p.len()).rev() {
                if p[j] > p[i] {
                    p.swap(i, j);
                    p[i + 1..].reverse();
                    return true;
                }
            }
        }
    }
    p.reverse();
    false
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
    fn next_permutation() {
        let mut a = [1, 2, 34, 56, 789];
        let mut cnt = 0;
        loop {
            cnt += 1;
            if !super::next_permutation(&mut a) {
                break;
            }
        }
        assert_eq!(cnt, 2 * 3 * 4 * 5);
        let mut a = b"aabbbc".to_owned();
        let mut cnt = 0;
        let mut set = std::collections::HashSet::new();
        loop {
            assert!(!set.contains(&a));
            set.insert(a.clone());
            cnt += 1;
            if !super::next_permutation(&mut a) {
                break;
            }
        }
        // 5C2 * 6C1
        assert_eq!(cnt, 10 * 6);
    }
}