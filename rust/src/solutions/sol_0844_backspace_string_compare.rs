#![allow(dead_code)]
pub fn backspace_compare(s: String, t: String) -> bool {
    fn backspacing(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        for ch in s.chars() {
            if ch != '#' {
                res.push(ch);
            } else {
                res.pop();
            }
        }
        res
    }

    backspacing(s) == backspacing(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare_basic() {
        assert!(backspace_compare("ab#c".to_string(), "ad#c".to_string()));
        assert!(backspace_compare("ab##".to_string(), "c#d#".to_string()));
        assert!(!backspace_compare("a#c".to_string(), "b".to_string()));
    }
}
