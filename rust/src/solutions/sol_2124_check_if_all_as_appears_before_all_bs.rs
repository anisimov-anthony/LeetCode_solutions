#![allow(dead_code)]
pub fn check_string(s: String) -> bool {
    let mut b_appears = false;
    for ch in s.chars() {
        if ch == 'b' {
            b_appears = true;
        } else {
            if b_appears == true {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_string_basic() {
        assert!(check_string("aaabbb".to_string()));
        assert!(!check_string("abab".to_string()));
        assert!(check_string("bbb".to_string()));
    }
}
