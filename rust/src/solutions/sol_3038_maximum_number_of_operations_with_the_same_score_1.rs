#[allow(dead_code)]
pub fn max_operations(nums: Vec<i32>) -> i32 {
    let mut new_nums = nums.clone();
    new_nums.reverse();
    let pref_sum = nums[0] + nums[1];
    let mut result = 0;
    while new_nums.len() > 1
        && new_nums[new_nums.len() - 1] + new_nums[new_nums.len() - 2] == pref_sum
    {
        result += 1;
        new_nums.pop();
        new_nums.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations_basic() {
        assert_eq!(max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(max_operations(vec![1, 5, 3, 3, 4, 1, 3, 2, 2, 3]), 2);
        assert_eq!(max_operations(vec![5, 3]), 1);
    }
}
