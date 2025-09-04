#![allow(dead_code)]
pub fn remove_duplicates(s: String) -> String {
    let mut stack = String::with_capacity(s.len());
    for ch in s.chars() {
        if stack.len() == 0 {
            stack.push(ch);
            continue;
        }
        if stack.chars().last().unwrap() == ch {
            stack.pop();
        } else {
            stack.push(ch);
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_basic() {
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca".to_string());
        assert_eq!(remove_duplicates("azxxzy".to_string()), "ay".to_string());
    }
}
