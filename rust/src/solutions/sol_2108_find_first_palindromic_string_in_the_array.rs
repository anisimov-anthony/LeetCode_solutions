#![allow(dead_code)]
pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words.iter() {
        if word
            .chars()
            .take(word.len() / 2)
            .eq(word.chars().rev().take(word.len() / 2))
        {
            return word.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_palindrome_basic() {
        assert_eq!(
            first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ]),
            "ada".to_string()
        );
        assert_eq!(
            first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            "racecar".to_string()
        );
        assert_eq!(
            first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
            String::new()
        );
    }
}
