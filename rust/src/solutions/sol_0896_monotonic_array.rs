#![allow(dead_code)]
pub fn is_monotonic(nums: Vec<i32>) -> bool {
    (nums.is_sorted_by(|a, b| a.le(&b))) || (nums.is_sorted_by(|a, b| b.le(&a)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_monotonic_basic() {
        assert!(is_monotonic(vec![1, 2, 2, 3]));
        assert!(is_monotonic(vec![6, 5, 4, 4]));
        assert!(!is_monotonic(vec![1, 3, 2]));
    }
}
