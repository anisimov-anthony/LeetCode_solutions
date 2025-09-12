#![allow(dead_code)]
pub fn count_vowel_strings(n: i32) -> i32 {
    (1..=n).fold(1, |acc, x| acc * (x + 4) / x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowel_strings_basic() {
        assert_eq!(count_vowel_strings(1), 5);
        assert_eq!(count_vowel_strings(2), 15);
        assert_eq!(count_vowel_strings(33), 66045);
    }
}
