#![allow(dead_code)]
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut res = 0;

    for i in 1..nums.len() {
        if nums[i] <= nums[i - 1] {
            let operations = nums[i - 1] + 1 - nums[i];
            res += operations;
            nums[i] = nums[i - 1] + 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_basic() {
        assert_eq!(min_operations(vec![1, 1, 1]), 3);
        assert_eq!(min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(min_operations(vec![8]), 0);
    }
}
