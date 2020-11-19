pub struct Pcg(u64);
const MUL: u64 = 5129263795064623965;
const INC: u64 = 4280768313201238837;
impl Pcg {
    pub fn new(seed: u64) -> Self {
        Self(seed.wrapping_add(INC))
    }
    pub fn next_u32(&mut self) -> u32 {
        // PCG-XSH-RR
        let mut x = self.0;
        self.0 = x.wrapping_mul(MUL).wrapping_add(INC);
        x ^= x >> 18;
        ((x >> 27) as u32).rotate_right((x >> 59) as u32)
    }
    pub fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | self.next_u32() as u64
    }
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> (32 - 23)) as f32 / (1 << 23) as f32
    }
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> (64 - 52)) as f64 / (1u64 << 52) as f64
    }
}
