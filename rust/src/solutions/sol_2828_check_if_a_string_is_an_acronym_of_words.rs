#![allow(dead_code)]
pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    let acronym: String = words
        .iter()
        .map(|word| word.chars().nth(0).unwrap())
        .collect();

    acronym == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_acronym_basic() {
        assert!(is_acronym(
            vec![
                "alice".to_string(),
                "bob".to_string(),
                "charlie".to_string()
            ],
            "abc".to_string()
        ));
        assert!(!is_acronym(
            vec!["an".to_string(), "apple".to_string()],
            "a".to_string()
        ));
        assert!(is_acronym(
            vec![
                "never".to_string(),
                "gonna".to_string(),
                "give".to_string(),
                "up".to_string(),
                "on".to_string(),
                "you".to_string()
            ],
            "ngguoy".to_string()
        ));
    }
}
