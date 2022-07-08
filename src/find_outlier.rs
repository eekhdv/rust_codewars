pub fn run(values: &[i32]) -> i32 {
    let (mut evens, mut odds, mut parity, mut x) = (0, 0, 0, 0);

    for i in 0..3 {
        if values[i] % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
    }

    if evens > odds {
        parity = 1
    }

    for value in values.into_iter() {
        x = *value;
        if (x % 2).abs() == parity {
            break;
        }
    }
    x
}
