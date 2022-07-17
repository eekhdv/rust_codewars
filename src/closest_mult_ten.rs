pub fn run(n: u32) -> u32 {
    return if n % 10 >= 5 { n - n % 10 + 10} else { n - n % 10}
    /* 
     * return (n + 5) / 10 * 10 
    */
}