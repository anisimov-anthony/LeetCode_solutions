#![allow(dead_code)]
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |agg, &x| agg | x) << (nums.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_xor_sum_basic() {
        assert_eq!(subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
