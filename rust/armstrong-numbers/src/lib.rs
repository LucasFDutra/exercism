pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();

    let mut sum: u32 = 0;

    for c in num_str.chars() {
        let digit = c.to_digit(10).unwrap();
        let power = num_str.len() as u32;
        sum += digit.pow(power);
    }

    if num == sum {
        return true;
    }
    return false;
}
