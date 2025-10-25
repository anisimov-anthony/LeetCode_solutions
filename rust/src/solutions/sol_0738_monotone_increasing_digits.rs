#![allow(dead_code)]
pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut m = n;
    let mut digits = Vec::new();
    while m != 0 {
        digits.push(m % 10);
        m = m / 10;
    }

    if digits.windows(2).all(|wd| wd[0] >= wd[1]) {
        return n;
    }

    digits.reverse();
    let mut incor_pos = 0;

    for i in (1..digits.len()).rev() {
        if digits[i - 1] > digits[i] {
            digits[i - 1] -= 1;
            incor_pos = i;
        }
    }

    for i in incor_pos..digits.len() {
        digits[i] = 9;
    }

    let result_str: String = digits.into_iter().map(|d| d.to_string()).collect();
    result_str.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotone_increasing_digits_basic() {
        assert_eq!(monotone_increasing_digits(10), 9);
        assert_eq!(monotone_increasing_digits(1234), 1234);
        assert_eq!(monotone_increasing_digits(332), 299);
    }
}
