pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut v_u: Vec<u32> = Vec::new();
	for i in 2..(integer / 2 + 1) {
        if integer % i == 0 {
            v_u.push(i);
        }
    }
    if v_u.is_empty() { Err(format!("{} is prime", integer)) } else { Ok(v_u) }
}