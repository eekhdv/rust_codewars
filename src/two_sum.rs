use core::num;

pub fn run(numbers: &[i32], target: i32) -> (usize, usize) {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}