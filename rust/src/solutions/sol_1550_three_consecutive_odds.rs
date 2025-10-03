#![allow(dead_code)]
pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.windows(3).any(|wd| wd.iter().all(|nm| nm % 2 == 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_consecutive_odds_basic() {
        assert!(!three_consecutive_odds(vec![2, 6, 4, 1]));
        assert!(three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]));
    }
}
