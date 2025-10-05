#![allow(dead_code)]
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let max = nums.iter().max().unwrap();
    (max + max + k - 1) * k / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximize_sum_basic() {
        assert_eq!(maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
