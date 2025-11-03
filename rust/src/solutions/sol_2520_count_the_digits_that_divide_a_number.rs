#![allow(dead_code)]
pub fn count_digits(num: i32) -> i32 {
    let digits: Vec<_> = num
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    digits.iter().filter(|dg| num % **dg as i32 == 0).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits_basic() {
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(121), 2);
        assert_eq!(count_digits(1248), 4);
    }
}
