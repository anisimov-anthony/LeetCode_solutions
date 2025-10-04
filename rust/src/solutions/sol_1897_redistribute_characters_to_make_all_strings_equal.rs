#![allow(dead_code)]
pub fn make_equal(words: Vec<String>) -> bool {
    let mut freqs = std::collections::HashMap::new();
    words.iter().for_each(|wd| {
        wd.chars().for_each(|ch| {
            freqs.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
        })
    });
    freqs.iter().all(|(_, fr)| *fr % words.len() == 0) || words.len() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_equal_basic() {
        assert!(make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]));
        assert!(!make_equal(vec!["ab".to_string(), "a".to_string()]));
    }
}
