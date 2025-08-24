#![allow(dead_code)]
pub fn score_of_string(s: String) -> i32 {
    if s.len() == 1 {
        return s.chars().nth(0).unwrap() as i32;
    }

    let mut res = 0;
    for i in 1..s.len() {
        res += (s.chars().nth(i).unwrap() as u32).abs_diff(s.chars().nth(i - 1).unwrap() as u32);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_string_basic() {
        assert_eq!(score_of_string("hello".to_string()), 13);
        assert_eq!(score_of_string("zaz".to_string()), 50);
    }
}
