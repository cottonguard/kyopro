pub struct WaveletMatrix(Box<[(BitVector, usize)]>);
impl WaveletMatrix {
    pub fn new(a: impl Into<Vec<u32>>) -> Self {
        let mut a = a.into();
        let n = a.len();
        let mut a_tmp = Vec::with_capacity(n);
        let mut mat = Vec::with_capacity(32);
        for b in (0..32).rev() {
            let mut bits = vec![0; n / 64 + 1];
            let mut cnt_ones = 0;
            for (i, x) in a.iter().enumerate() {
                bits[i / 64] |= ((x >> b & 1) as u64) << i % 64;
                cnt_ones += x >> b & 1;
            }
            mat.push((BitVector::new(bits), n - cnt_ones as usize));
            a_tmp.splice(
                ..,
                a.iter()
                    .filter(|&x| x >> b & 1 == 0)
                    .chain(a.iter().filter(|&x| x >> b & 1 == 1))
                    .copied(),
            );
            std::mem::swap(&mut a, &mut a_tmp);
        }
        mat.reverse();
        Self(mat.into())
    }
    pub fn rank(&self, mut l: usize, mut r: usize, x: u32) -> usize {
        for d in (0..32).rev() {
            let (bv, cnt_zeros) = &self.0[d];
            if x >> d & 1 == 0 {
                l = bv.rank_zero(l);
                r = bv.rank_zero(r);
            } else {
                l = cnt_zeros + bv.rank_one(l);
                r = cnt_zeros + bv.rank_one(r);
            }
        }
        r - l
    }
    pub fn quantile(&self, mut l: usize, mut r: usize, mut k: usize) -> u32 {
        let mut x = 0;
        for d in (0..32).rev() {
            let (bv, cnt_zeros) = &self.0[d];
            let l0 = bv.rank_zero(l);
            let r0 = bv.rank_zero(r);
            if k < r0 - l0 {
                l = l0;
                r = r0;
            } else {
                x |= 1 << d;
                k -= r0 - l0;
                l = cnt_zeros + bv.rank_one(l);
                r = cnt_zeros + bv.rank_one(r);
            }
        }
        x
    }
}
struct BitVector {
    sum: Vec<u32>,
    bits: Vec<u64>,
}
impl BitVector {
    fn new(bits: Vec<u64>) -> Self {
        Self {
            sum: std::iter::once(0)
                .chain(bits.iter().map(|x| x.count_ones()).scan(0u32, |s, x| {
                    *s += x;
                    Some(*s)
                }))
                .collect(),
            bits,
        }
    }
    fn rank_one(&self, r: usize) -> usize {
        (self.sum[r / 64] + (self.bits[r / 64] & (1 << r % 64) - 1).count_ones()) as usize
    }
    fn rank_zero(&self, r: usize) -> usize {
        r - self.rank_one(r)
    }
    /*
    fn select_one(&self, n: usize) -> usize {
        let mut l = -1;
        let mut r = 64 * self.bits.len() as isize;
        while r - l > 1 {
            let h = (l + r) / 2;
            if self.rank_one(h as usize) >= n {
                r = h;
            } else {
                l = h;
            }
        }
        r as usize
    }
    */
}
