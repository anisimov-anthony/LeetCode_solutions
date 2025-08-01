#![allow(dead_code)]
pub fn balanced_string_split(s: String) -> i32 {
    let mut balance = 0;
    let mut result = 0;

    for c in s.chars() {
        match c {
            'R' => balance += 1,
            'L' => balance -= 1,
            _ => continue,
        }

        if balance == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_string_split_basic() {
        assert_eq!(balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(balanced_string_split("RLRRRLLRLL".to_string()), 2);
        assert_eq!(balanced_string_split("LLLLRRRR".to_string()), 1);
    }
}
