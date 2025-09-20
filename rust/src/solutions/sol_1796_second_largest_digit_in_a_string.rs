#![allow(dead_code)]
pub fn second_highest(s: String) -> i32 {
    let mut digits = std::collections::HashSet::new();

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digits.insert(ch.to_digit(10).unwrap() as i32);
        }
    }

    if digits.len() < 2 {
        return -1;
    }

    let mut v: Vec<i32> = digits.into_iter().collect();
    v.sort_unstable();

    v[v.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_highest_basic() {
        assert_eq!(second_highest("dfa12321afd".to_string()), 2);
        assert_eq!(second_highest("abc1111".to_string()), -1);
    }
}
