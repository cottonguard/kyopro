pub fn time<F: Fn() -> T, T>(desc: &str, f: F) -> T {
    eprintln!("begin {}", desc);
    let begin = std::time::Instant::now();
    let res = f();
    eprintln!("end {} in {:?}", desc, begin.elapsed());
    res
}