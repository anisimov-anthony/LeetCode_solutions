#![allow(dead_code)]
pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(&a));
    let a = nums[0];
    let b = nums[1];
    let c = nums[nums.len() - 1];
    let d = nums[nums.len() - 2];

    return a * b - c * d;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_difference_basic() {
        assert_eq!(max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
    }
}
