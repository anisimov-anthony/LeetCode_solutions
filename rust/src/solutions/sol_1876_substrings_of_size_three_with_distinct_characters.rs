#![allow(dead_code)]
pub fn count_good_substrings(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    s.windows(3)
        .filter(|wd| wd[0] != wd[1] && wd[1] != wd[2] && wd[0] != wd[2])
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_substrings_basic() {
        assert_eq!(count_good_substrings("xyzzaz".to_string()), 1);
        assert_eq!(count_good_substrings("aababcabc".to_string()), 4);
    }
}
