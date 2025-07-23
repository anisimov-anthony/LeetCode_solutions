#![allow(dead_code)]
pub fn maximum69_number(num: i32) -> i32 {
    let mut digits: Vec<_> = num
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    for i in 0..digits.len() {
        if digits[i] == 6 {
            digits[i] = 9;
            break;
        }
    }

    digits.iter().fold(0, |acc, elem| acc * 10 + *elem as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum69_number_basic() {
        assert_eq!(maximum69_number(9669), 9969);
        assert_eq!(maximum69_number(9996), 9999);
        assert_eq!(maximum69_number(9999), 9999);
    }
}
