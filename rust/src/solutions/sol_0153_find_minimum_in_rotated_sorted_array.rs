#![allow(dead_code)]
pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left != right - 1 {
        let mid = (left + right) / 2;
        if nums[mid] > nums[right] {
            left = mid;
        } else {
            right = mid;
        }
    }
    nums[left].min(nums[right])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_basic() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }
}
