#![allow(dead_code)]
pub fn count_odds(low: i32, high: i32) -> i32 {
    (high - low) / 2 + (high % 2 | low % 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_odds_basic() {
        assert_eq!(count_odds(3, 7), 3);
        assert_eq!(count_odds(8, 10), 1);
    }
}
