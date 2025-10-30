#![allow(dead_code)]
pub fn check(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    if nums.is_sorted() {
        return true;
    }
    let mut ch_idx = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] - nums[i + 1] > 0 {
            ch_idx = i + 1;
            break;
        }
    }

    nums.rotate_left(ch_idx);
    return nums.is_sorted() || nums.is_sorted_by(|a, b| a.gt(&b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_basic() {
        assert!(check(vec![3, 4, 5, 1, 2]));
        assert!(!check(vec![2, 1, 3, 4]));
        assert!(check(vec![1, 2, 3]));
    }
}
