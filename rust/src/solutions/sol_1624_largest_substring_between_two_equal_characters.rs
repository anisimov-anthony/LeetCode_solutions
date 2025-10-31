#![allow(dead_code)]
pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut result = -1;
    let s: Vec<_> = s.chars().collect();
    let mut positions = std::collections::HashMap::new();

    for (i, ch) in s.iter().enumerate() {
        positions.entry(ch).or_insert_with(Vec::new).push(i);
    }

    for (_, poses) in positions.iter() {
        result = result.max(poses[poses.len() - 1] as i32 - poses[0] as i32 - 1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length_between_equal_characters_basic() {
        assert_eq!(max_length_between_equal_characters("aa".to_string()), 0);
        assert_eq!(max_length_between_equal_characters("abca".to_string()), 2);
        assert_eq!(max_length_between_equal_characters("cbzxy".to_string()), -1);
    }
}
