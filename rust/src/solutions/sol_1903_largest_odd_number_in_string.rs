#![allow(dead_code)]
pub fn largest_odd_number(num: String) -> String {
    for i in (0..num.len()).rev() {
        if (num.as_bytes()[i] - b'0') % 2 == 1 {
            return num[0..i + 1].to_string();
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_odd_number_basic() {
        assert_eq!(largest_odd_number("52".to_string()), "5".to_string());
        assert_eq!(largest_odd_number("4206".to_string()), String::new());
        assert_eq!(largest_odd_number("35427".to_string()), "35427".to_string());
    }
}
