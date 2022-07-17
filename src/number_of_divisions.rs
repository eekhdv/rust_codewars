use crate::is_square::run;

pub fn run(n: u32, divisor: u32) -> u32 {
    let (mut i, mut _n): (u32, u32) = (0, n);
    while _n / divisor != 0 {
        _n /= divisor;
        i += 1;
    }
    i
}