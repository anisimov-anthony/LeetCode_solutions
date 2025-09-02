#![allow(dead_code)]
pub fn remove_outer_parentheses(s: String) -> String {
    let mut res = String::with_capacity(s.len());
    let mut position = 0;

    for ch in s.chars() {
        if ch == '(' {
            if position > 0 {
                res.push(ch);
            }
            position += 1;
        } else {
            position -= 1;
            if position > 0 {
                res.push(ch);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses_basic() {
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(
            remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(remove_outer_parentheses("()()".to_string()), String::new());
    }
}
