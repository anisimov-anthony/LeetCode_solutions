#![allow(dead_code)]
pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut res = 0;
    while left < right {
        res = res.max(nums[left] + nums[right]);
        left += 1;
        right -= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_pair_sum_basic() {
        assert_eq!(min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
