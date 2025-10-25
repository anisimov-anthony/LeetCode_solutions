#![allow(dead_code)]
pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let full_sum: i32 = nums.iter().sum();
    let mut left_sum = 0;

    for i in 0..nums.len() {
        if left_sum == full_sum - left_sum - nums[i] {
            return i as i32;
        }
        left_sum += nums[i];
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_middle_index_basic() {
        assert_eq!(find_middle_index(vec![2, 3, -1, 8, 4]), 3);
        assert_eq!(find_middle_index(vec![1, -1, 4]), 2);
        assert_eq!(find_middle_index(vec![2, 5]), -1);
    }
}
