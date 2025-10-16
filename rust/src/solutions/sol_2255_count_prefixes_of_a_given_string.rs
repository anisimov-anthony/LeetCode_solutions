#![allow(dead_code)]
pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    words.iter().filter(|&wd| s.starts_with(wd)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefixes_basic() {
        assert_eq!(
            count_prefixes(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
        assert_eq!(
            count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()),
            2
        );
    }
}
