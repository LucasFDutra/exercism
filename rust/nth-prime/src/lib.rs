pub fn is_prime(n: u32) -> bool {
    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;
    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    return candidate;
}
