pub fn square_of_sum(n: u32) -> u32 {
    let sum = ((1 + n) * n) / 2;
    return sum * sum;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i * i;
    }
    return total;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
