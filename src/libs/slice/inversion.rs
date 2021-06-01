use std::{mem::ManuallyDrop, ptr};
pub fn inversion_number<T: Ord>(a: &mut [T]) -> u64 {
    rec(a, &mut Vec::with_capacity(a.len()))
}
fn rec<T: Ord>(a: &mut [T], temp: &mut Vec<ManuallyDrop<T>>) -> u64 {
    let len = a.len();
    if len <= 1 {
        return 0;
    }
    if len <= 10 {
        return insertion_sort(a);
    }
    let mid = len / 2;
    let mut res = rec(&mut a[..mid], temp) + rec(&mut a[mid..], temp);
    let mut l = 0;
    let mut r = mid;
    debug_assert!(temp.is_empty());
    while l < mid || r < len {
        let pos;
        if r >= a.len() || l < mid && a[l] <= a[r] {
            pos = l;
            l += 1;
        } else {
            res += (mid - l) as u64;
            pos = r;
            r += 1;
        }
        unsafe {
            temp.push(ManuallyDrop::new(ptr::read(&a[pos])));
        }
    }
    unsafe {
        ptr::copy_nonoverlapping(temp.as_ptr() as *const T, a.as_mut_ptr(), a.len());
    }
    temp.clear();
    res
}
fn insertion_sort<T: Ord>(a: &mut [T]) -> u64 {
    let mut inv = 0;
    for i in 1..a.len() {
        unsafe {
            let temp = ptr::read(&a[i]);
            let mut j = i;
            while j > 0 {
                if a[j - 1] <= temp {
                    break;
                }
                ptr::copy_nonoverlapping(&a[j - 1], &mut a[j], 1);
                j -= 1;
            }
            ptr::write(&mut a[j], temp);
            inv += (i - j) as u64;
        }
    }
    inv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::*;

    #[test]
    fn inversion_number_test() {
        let mut rand = Pcg::seed_from_u64(838383);

        let mut a = vec![];
        for n in 1..=50 {
            a.clear();
            for _ in 0..n {
                a.push(rand.range(0, 10));
            }

            let mut naive = 0;
            for i in 0..a.len() {
                for j in 0..i {
                    if a[i] < a[j] {
                        naive += 1;
                    }
                }
            }

            assert_eq!(inversion_number(&mut a), naive);
            // assert!(a.is_sorted());
        }
    }
}
