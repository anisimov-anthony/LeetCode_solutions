#![allow(dead_code)]
pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 2 {
        return (nums[0] - 1) * (nums[1] - 1);
    }

    let mut nums = nums;
    nums.sort();
    (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_basic() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(max_product(vec![3, 7]), 12);
    }
}
