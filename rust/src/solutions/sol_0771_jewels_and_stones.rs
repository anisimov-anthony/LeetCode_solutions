#![allow(dead_code)]
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels: std::collections::HashSet<_> = jewels.chars().into_iter().collect();

    let mut result = 0;
    for stone in stones.chars() {
        if jewels.contains(&stone) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones_basic() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}
