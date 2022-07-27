fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b
    }
    if b == 0 || a == b{
        return a
    }
    if a > b { gcd(b, a % b) } else { gcd(a, b % a) }
}

pub fn run(s: &str) -> String {
    let mut decoded = String::new();
    let enlglish_alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut x = 0;
    for ch in s.chars() {
        if ch.is_numeric() {
            x *= 10;
            x += ch.to_digit(10).expect("Error!");
        } else {
            if gcd(x, 26) != 1 {
                decoded.push_str("Impossible to decode");
                break;
            }
            let cur_letter = enlglish_alphabet.find(ch).expect("Is not a letter");
            for i in 0..=25 {
                if x * i % 26 == cur_letter as u32 {
                    decoded.push(enlglish_alphabet.chars().nth(i as usize).expect("Error!"));
                    break;
                }
            }
        }
    }
    decoded
}