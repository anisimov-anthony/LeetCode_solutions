#![allow(dead_code)]
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    words
        .iter()
        .filter(|&word| word.chars().all(|ch| allowed.contains(ch)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_consistent_strings_basic() {
        assert_eq!(
            count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            ),
            7
        );
        assert_eq!(
            count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            ),
            4
        );
    }
}
