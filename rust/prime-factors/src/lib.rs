pub fn get_divisor(n: u64) -> u64 {
    for i in 2..=n {
        if n % i == 0 {
            return i;
        }
    }
    return n;
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut n_ = n;

    if n < 2 {
        return result;
    }

    loop {
        let divisor = get_divisor(n_);
        result.push(divisor);
        if divisor == n_ {
            break;
        }
        n_ /= divisor;
    }
    return result;
}
