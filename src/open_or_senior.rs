pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for pair in data {
        if pair.0 >= 55 && pair.1 > 7 {
            result.push("Senior".to_string());
        } else {
            result.push("Open".to_string());
        }
    }
    result
}
