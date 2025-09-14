// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut sum: u32 = 1;
    for i in 1.. n {
        sum += sum*i
    }
    sum
    /* TODO */
}
