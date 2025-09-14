#![allow(dead_code)]
pub fn make_fancy_string(s: String) -> String {
    if s.len() < 2 {
        return s;
    }

    let mut res = String::with_capacity(s.len());
    let mut last = '_';
    let mut counter = 0;
    for ch in s.chars() {
        if ch == last {
            if counter < 1 {
                res.push_str(&ch.to_string());
            }
            counter += 1;
        } else {
            res.push_str(&ch.to_string());
            counter = 0;
        }
        last = ch;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string_basic() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }
}
