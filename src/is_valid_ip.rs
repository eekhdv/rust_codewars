fn between(n: i32, left: i32, right: i32) -> bool {
    n >= left && n <= right
}

fn is_only_nums(n: &str) -> bool {
    for ch in n.chars() {
        match ch {
            '0'..='9' => continue,
            _ => return false,
        }
    }
    return true
}

fn is_correct_num(n: &str) -> bool {
    (n.len() > 1 && n.chars().nth(0) != Some('0') || n.len() == 1) && is_only_nums(n)
}

pub fn run(ip: &str) -> bool {
    let ip_nums = ip.split('.');
    let mut i = 0;
    for num in ip_nums {
        if !is_correct_num(num) || !between(num.parse::<i32>().unwrap(), 0, 255) {
            return false;
        }
        i += 1;
    }
    i == 4
}
