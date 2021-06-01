use super::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pcg(u64);
const MUL: u64 = 5129263795064623965;
const INC: u64 = 4280768313201238837;
impl Pcg {
    pub fn seed_from_u64(seed: u64) -> Self {
        Self(seed.wrapping_add(INC))
    }
}
impl Rng for Pcg {
    fn next_u32(&mut self) -> u32 {
        // PCG-XSH-RR
        let mut x = self.0;
        self.0 = x.wrapping_mul(MUL).wrapping_add(INC);
        x ^= x >> 18;
        ((x >> 27) as u32).rotate_right((x >> 59) as u32)
    }
    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | self.next_u32() as u64
    }
}
