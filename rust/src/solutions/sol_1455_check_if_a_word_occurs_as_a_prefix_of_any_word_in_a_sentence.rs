#![allow(dead_code)]
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let sentence: Vec<_> = sentence.split_whitespace().collect();
    sentence
        .iter()
        .enumerate()
        .find(|&(_, wd)| wd.starts_with(&search_word))
        .map(|(i, _)| i as i32 + 1)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prefix_of_word_basic() {
        assert_eq!(
            is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
        assert_eq!(
            is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            ),
            2
        );
        assert_eq!(
            is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
