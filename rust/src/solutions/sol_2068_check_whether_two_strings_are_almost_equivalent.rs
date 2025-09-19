#![allow(dead_code)]
pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut first = std::collections::HashMap::new();
    let mut second = std::collections::HashMap::new();

    for ch in word1.chars() {
        first.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
    }
    for ch in word2.chars() {
        second.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
    }

    for ch in 'a'..='z' {
        let f1 = *first.get(&ch).unwrap_or(&0);
        let f2 = *second.get(&ch).unwrap_or(&0);
        if (f1 as i32 - f2 as i32).abs() > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_almost_equivalent_basic() {
        assert!(!check_almost_equivalent(
            "aaaa".to_string(),
            "bccb".to_string()
        ));
        assert!(check_almost_equivalent(
            "abcdeef".to_string(),
            "abaaacc".to_string()
        ));
        assert!(check_almost_equivalent(
            "cccddabba".to_string(),
            "babababab".to_string()
        ));
    }
}
