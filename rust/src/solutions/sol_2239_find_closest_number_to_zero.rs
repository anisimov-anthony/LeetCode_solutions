#[allow(dead_code)]
pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];

    for i in 1..nums.len() {
        if nums[i].abs() < result.abs() || (nums[i].abs() == result.abs() && nums[i] > result) {
            result = nums[i];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_number_basic() {
        assert_eq!(find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
        assert_eq!(find_closest_number(vec![2, -1, 1]), 1);
    }
}
