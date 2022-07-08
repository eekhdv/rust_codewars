pub fn run(n: i64) -> bool {
    let result = f64::sqrt(n as f64);
    result.fract() == 0.0
}
