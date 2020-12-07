use super::*;

pub fn mod_inv_table<M: Modulo>(n: usize) -> Vec<ModInt<M>> {
    let mut inv = vec![ModInt::new(0); n + 1];
    inv[1] = ModInt::new(1);
    for x in 2..=n {
        let div = M::modulo() / x as u32;
        let rem = M::modulo() % x as u32;
        inv[x] = inv[rem as usize] * -ModInt::new(div);
    }
    inv
}
