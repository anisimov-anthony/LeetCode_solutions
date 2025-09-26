#![allow(dead_code)]
pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut counter = 0;
    let mut repeating_pattern = word.clone();
    while sequence.contains(&repeating_pattern) {
        counter += 1;
        repeating_pattern.push_str(&word);
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_repeating_basic() {
        assert_eq!(max_repeating("ababc".to_string(), "ab".to_string()), 2);
        assert_eq!(max_repeating("ababc".to_string(), "ba".to_string()), 1);
        assert_eq!(max_repeating("ababc".to_string(), "ac".to_string()), 0);
    }
}
