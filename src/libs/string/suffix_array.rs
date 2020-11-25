/// O(N(log N)^2) time.
pub fn suffix_array<T: Into<usize> + Copy>(s: &[T]) -> Vec<usize> {
    let mut rank: Vec<_> = s.iter().map(|c| (*c).into()).collect();
    let mut sa: Vec<_> = (0..s.len()).collect();
    sa.sort_by(|&i, &j| rank[i].cmp(&rank[j]).then_with(|| j.cmp(&i)));
    let mut tmp = vec![0; s.len()];
    for k in (0..).map(|i| 1 << i).take_while(|k| *k < s.len()) {
        sa.sort_by_key(|&i| (rank[i], rank.get(i + k)));
        tmp[sa[0]] = 0;
        for i in 1..s.len() {
            tmp[sa[i]] =
                if rank[sa[i]] > rank[sa[i - 1]] || rank.get(sa[i] + k) > rank.get(sa[i - 1] + k) {
                    i
                } else {
                    tmp[sa[i - 1]]
                };
        }
        std::mem::swap(&mut rank, &mut tmp);
    }
    sa
}

#[cfg(test)]
mod tests {
    #[test]
    fn suffix_array() {
        let sa = super::suffix_array(b"mississippi");
        assert_eq!(sa, [10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
    }
}
