#![allow(dead_code)]
pub fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(idx) = word.find(ch) {
        return word[..=idx].chars().rev().collect::<String>() + &word[(idx + 1)..];
    }
    word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_prefix_basic() {
        assert_eq!(
            reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd".to_string()
        );
        assert_eq!(
            reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe".to_string()
        );
        assert_eq!(reverse_prefix("abcd".to_string(), 'z'), "abcd".to_string());
    }
}
