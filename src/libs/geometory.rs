pub fn argument_sort(a: &mut [(i32, i32)]) {
    a.sort_by(|&(x1, y1), &(x2, y2)| {
        (y1 >= 0)
            .cmp(&(y2 >= 0))
            .then_with(|| (x2 as i64 * y1 as i64).cmp(&(x1 as i64 * y2 as i64)))
            .then_with(|| y1.cmp(&y2)) // (0, 0) < (x, y (> 0))
            .then_with(|| x2.cmp(&x1)) // (x1 (>= 0), 0) < (x2 (< 0), 0)
    });
}

use std::ops;

pub fn graham_scan<T>(ps: &[[T; 2]]) -> Vec<usize>
where
    T: PartialOrd + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy,
{
    let mut ids: Vec<_> = (0..ps.len()).collect();
    ids.sort_by(|&i, &j| ps[i].partial_cmp(&ps[j]).unwrap());
    let mut hull: Vec<usize> = Vec::new();
    for dir in 0..2 {
        let mut len = dir;
        for &i in &ids {
            let [x2, y2] = ps[i];
            while len >= 2 {
                let [x0, y0] = ps[hull[hull.len() - 2]];
                let [x1, y1] = ps[hull[hull.len() - 1]];
                if (x1 - x0) * (y2 - y0) >= (x2 - x0) * (y1 - y0) {
                    break;
                }
                hull.pop();
                len -= 1;
            }
            hull.push(i);
            len += 1;
        }
        ids.pop();
        ids.reverse();
        ids.pop();
    }
    hull
}

pub fn farthest_pair<T>(ps: &[[T; 2]]) -> (usize, usize)
where
    T: PartialOrd + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy,
{
    let norm = |[x0, y0]: [T; 2], [x1, y1]: [T; 2]| (x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0);
    let hull = graham_scan(ps);
    let mut res = (0, 0);
    let mut max = None;
    let mut j = 0;
    for &i in &hull {
        let mut d = norm(ps[i], ps[hull[j]]);
        loop {
            let j_inc = if j + 1 < hull.len() { j + 1 } else { 0 };
            let d_inc = norm(ps[i], ps[hull[j_inc]]);
            if d >= d_inc {
                break;
            }
            d = d_inc;
            j = j_inc;
        }
        if Some(d) > max {
            max = Some(d);
            res = (i, hull[j]);
        }
    }
    res
}
