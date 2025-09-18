#![allow(dead_code)]
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    (0..nums[0].len())
        .map(|i| (nums[i].as_bytes()[i] ^ 1) as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_different_binary_string_basic() {
        assert_eq!(
            find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
            "11".to_string()
        );
        assert_eq!(
            find_different_binary_string(vec!["00".to_string(), "01".to_string()]),
            "10".to_string()
        );
        assert_eq!(
            find_different_binary_string(vec![
                "111".to_string(),
                "011".to_string(),
                "001".to_string()
            ]),
            "000".to_string()
        );
    }
}
