use super::*;

#[test]
fn mod_int() {
    type Mint = ModInt<Mod1e9p7>;
    let x = Mint::new(57577);
    assert_eq!(x / x, Mint::new(1));
    assert_eq!(x.half() * Mint::new(2), x);
    assert_eq!(x.pow((Mint::modulo() - 1) as u64 * 1234567), Mint::new(1));
    assert_eq!(x.pow((Mint::modulo() - 1) as i64 * -1234567), Mint::new(1));
    assert_eq!(Mint::from(-1000) + Mint::from(1000), Mint::new(0));
    assert_eq!(Mint::from(17i8), Mint::new(17));
    assert_eq!(Mint::from(-10i8), Mint::new(Mint::modulo() - 10));
}

#[test]
fn var_mod() {
    set_var_mod(7);
    assert_eq!(VarMod::modulo(), 7);
    set_var_mod(13);
    assert_eq!(VarMod::modulo(), 13);
}

#[test]
fn binom() {
    const N: usize = 20;
    let f = fact::Fact::<Mod1e9p7>::new(N);
    let mut pascal = [[ModInt::new(0); N + 1]; N + 1];
    for i in 0..=N {
        pascal[i][0] = ModInt::new(1);
        for j in 1..=i {
            pascal[i][j] = pascal[i - 1][j - 1] + pascal[i - 1][j];
        }
    }
    for i in 0..=N {
        for j in 0..=N {
            assert_eq!(f.binom(i, j), pascal[i][j]);
        }
    }
}
