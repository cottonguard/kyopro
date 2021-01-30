use super::Graph;

pub fn tsort(g: &Graph) -> Option<Vec<usize>> {
    let mut res = vec![0; g.len()];
    let mut i = res.len();
    let mut stack = Vec::new();
    let mut state = vec![0u8; g.len()];
    for u in (0..g.len()).rev() {
        debug_assert!(stack.is_empty());
        if state[u] != 0 {
            debug_assert_eq!(state[u], 2);
            continue;
        }
        stack.push(u);
        while let Some(u) = stack.pop() {
            if u as isize >= 0 {
                if state[u] != 0 {
                    continue;
                }
                state[u] = 1;
                stack.push(!u);
                for &v in g[u].iter().rev() {
                    match state[v] {
                        0 => stack.push(v),
                        1 => return None,
                        _ => {}
                    }
                }
            } else {
                let u = !u;
                state[u] = 2;
                i -= 1;
                res[i] = u;
            }
        }
    }
    debug_assert_eq!(i, 0);
    Some(res)
}
