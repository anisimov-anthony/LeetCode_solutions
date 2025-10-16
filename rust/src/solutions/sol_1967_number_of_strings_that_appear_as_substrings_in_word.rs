#![allow(dead_code)]
pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    patterns.iter().filter(|&pt| word.contains(pt)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_strings_basic() {
        assert_eq!(
            num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
        assert_eq!(
            num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aaaaabbbbb".to_string()
            ),
            2
        );
        assert_eq!(
            num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string()],
                "ab".to_string()
            ),
            3
        );
    }
}
