pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2_u32];
    let mut x = 3;
    
    while primes.len() != n as usize + 1 {
        if primes.iter().all(|y| x % y != 0) {
            primes.push(x)
        }
        x += 2;
    }

    *primes.last().unwrap()
}
