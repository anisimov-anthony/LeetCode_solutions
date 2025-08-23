#![allow(dead_code)]
pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.concat() == word2.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_strings_are_equal_basic() {
        assert!(array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ));

        assert!(!array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ));

        assert!(array_strings_are_equal(
            vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
            vec!["abcddefg".to_string()]
        ));
    }
}
