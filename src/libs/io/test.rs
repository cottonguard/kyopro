use super::*;
#[test]
fn input() {
    let s = b"123 -3230     abcd \r\n  efgh\n12.34";
    let mut kin = KInput::new(s as &[u8]);
    assert_eq!(kin.input::<(u32, i32)>(), (123, -3230));
    assert_eq!(kin.input::<Vec<u8>>(), b"abcd");
    assert_eq!(kin.input::<String>(), "efgh");
    assert!((kin.input::<f64>() - 12.34).abs() < 1e-15);
    assert_eq!(kin.bytes(), b"");
}

#[test]
fn collect() {
    let s = b"1  2 3\n4 5";
    let mut kin = KInput::new(s as &[u8]);
    let a: Vec<i32> = kin.collect(5);
    assert_eq!(a, [1, 2, 3, 4, 5]);
}

#[test]
fn output_int() {
    let mut out = KOutput::new(Vec::<u8>::new());
    let mut out_fmt = Vec::<u8>::new();
    let mut x = 0;
    for i in 1..10 {
        x = 10 * x + i;
        let y = if i % 3 == 0 { -x } else { x };
        out.output(&y);
        if i > 1 {
            out_fmt.push(b' ');
        }
        out_fmt.extend_from_slice(format!("{}", y).as_bytes());
    }
    // dbg!(String::from_utf8_lossy(&out));
    assert_eq!(out.inner(), &out_fmt);
}

#[test]
fn output_int_seq() {
    let a: Vec<_> = (-10..=10).collect();
    let mut out = KOutput::new(Vec::<u8>::new());
    out.seq(&a);
    let mut out_fmt = Vec::new();
    for (i, x) in a.into_iter().enumerate() {
        if i > 0 {
            out_fmt.push(b' ');
        }
        out_fmt.extend_from_slice(format!("{}", x).as_bytes());
    }
    assert_eq!(out.inner(), &out_fmt);
}
