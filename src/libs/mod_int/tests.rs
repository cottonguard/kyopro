use super::*;

#[test]
fn mod_int() {
    type Mint = ModInt<Mod1e9p7>;
    let x = Mint::new(57577);
    assert_eq!(x / x, Mint::new(1));
    assert_eq!(x.half() * Mint::new(2), x);
}

#[test]
fn var_mod() {
    set_var_mod(7);
    assert_eq!(VarMod::modulo(), 7);
    set_var_mod(13);
    assert_eq!(VarMod::modulo(), 13);
}
