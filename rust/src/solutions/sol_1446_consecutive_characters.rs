#![allow(dead_code)]
pub fn max_power(s: String) -> i32 {
    let mut res = 1;
    let mut current_count = 1;
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            current_count += 1;
            res = res.max(current_count);
        } else {
            current_count = 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_power_basic() {
        assert_eq!(max_power("leetcode".to_string()), 2);
        assert_eq!(max_power("abbcccddddeeeeedcba".to_string()), 5);
    }
}
