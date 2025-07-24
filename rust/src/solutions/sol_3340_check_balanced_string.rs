#![allow(dead_code)]
pub fn is_balanced(num: String) -> bool {
    let mut odd = 0;
    let mut even = 0;
    for (idx, val) in num.chars().into_iter().enumerate() {
        match idx % 2 == 0 {
            true => even += val.to_digit(10).unwrap(),
            false => odd += val.to_digit(10).unwrap(),
        }
    }

    odd == even
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced_basic() {
        assert!(!is_balanced("1234".to_string()));
        assert!(is_balanced("24123".to_string()));
    }
}
