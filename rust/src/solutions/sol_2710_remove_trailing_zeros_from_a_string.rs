#![allow(dead_code)]
pub fn remove_trailing_zeros(num: String) -> String {
    let mut num = num;
    while let Some(last) = num.chars().last() {
        if last == '0' {
            num.pop();
        } else {
            return num;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_trailing_zeros_basic() {
        assert_eq!(
            remove_trailing_zeros("51230100".to_string()),
            "512301".to_string()
        );
        assert_eq!(remove_trailing_zeros("123".to_string()), "123".to_string());
    }
}
