use std::vec;

fn to_digits(code: &str) -> Vec<u32> {
    let mut code_numbers: Vec<u32> = vec![];
    for c in code.chars() {
        if c.is_ascii_digit() {
            code_numbers.push(c.to_digit(10).unwrap());
        } else if c != ' ' {
            return vec![];
        }
    }
    return code_numbers;
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_numbers: Vec<u32> = to_digits(code);
    if code_numbers.len() <= 1 {
        return false;
    }

    let mut code_sum: u32 = 0;
    for (i, n) in code_numbers.iter().rev().enumerate() {
        if i % 2 == 1 {
            let mut double = n * 2;
            if double > 9 {
                double -= 9;
            }
            code_sum += double;
        } else {
            code_sum += n;
        }
    }

    if code_sum % 10 == 0 {
        return true;
    }
    return false;
}
