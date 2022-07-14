pub fn run(s: &str) -> Vec<String> {
    let mut i = 0;
    let mut res = String::new();
    let mut split_string: Vec<String> = Vec::new();

    for chr in s.chars() {
        res.push(chr);
        i += 1;
        if i == 2 {
            split_string.push(res);
            res = "".to_string();
            i = 0;
        }
    }
    if i == 1 {
        res.push('_');
    }
    split_string.push(res);
    split_string
}