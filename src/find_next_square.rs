pub fn run(sq: u64) -> Option<u64> {
    let res = (sq as f64).sqrt();
    if res.floor() == res {
        Some((res as u64 + 1).pow(2))
    } else {
        None
    }
}
