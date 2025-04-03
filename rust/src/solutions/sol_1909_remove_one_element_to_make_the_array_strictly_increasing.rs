#[allow(dead_code)]
pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    pub fn is_increasing(nums: Vec<i32>) -> bool {
        nums.is_sorted_by(|a, b| a < b)
    }

    for i in 0..nums.len() {
        if i == 0 {
            if is_increasing(nums[1..nums.len()].to_vec()) {
                return true;
            }
        } else if i == nums.len() - 1 {
            if is_increasing(nums[0..nums.len() - 1].to_vec()) {
                return true;
            }
        } else {
            if is_increasing(nums[0..i].to_vec())
                && is_increasing(nums[i + 1..nums.len()].to_vec())
                && nums[i - 1] < nums[i + 1]
            {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_increasing_basic() {
        assert_eq!(can_be_increasing(vec![1, 2, 10, 5, 7]), true);
        assert_eq!(can_be_increasing(vec![2, 3, 1, 2]), false);
        assert_eq!(can_be_increasing(vec![1, 1, 1]), false);
    }
}
