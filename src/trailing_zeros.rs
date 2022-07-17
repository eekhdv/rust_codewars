pub fn run(n: u64) -> u64 {
    let (mut zeros_num, mut temp_fac): (u64, u64) = (0, 1);    
    for i in 1..(n + 1) {
        temp_fac *= i;
        loop {
            temp_fac = if temp_fac % 10 == 0 { temp_fac / 10 } else { break; };
            zeros_num += 1;
        }
        temp_fac %= 1000000;
    }
    zeros_num 
}