#![allow(dead_code)]
pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut converted: String = s.chars().map(|ch| (ch as u8 - 96).to_string()).collect();
    let mut res = 0;
    for _ in 0..k {
        res = converted
            .chars()
            .fold(0, |acc, ch| acc + ch.to_digit(10).unwrap() as i32);
        converted = res.to_string();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lucky_basic() {
        assert_eq!(get_lucky("iiii".to_string(), 1), 36);
        assert_eq!(get_lucky("leetcode".to_string(), 2), 6);
        assert_eq!(get_lucky("zbax".to_string(), 2), 8);
    }
}
