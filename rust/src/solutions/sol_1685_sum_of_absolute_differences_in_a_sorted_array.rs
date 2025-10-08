#![allow(dead_code)]
pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    res[0] = nums.iter().sum::<i32>() - nums[0] * nums.len() as i32;

    for i in 1..nums.len() {
        res[i] = res[i - 1] + (nums[i] - nums[i - 1]) * (2 * i as i32 - nums.len() as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_absolute_differences_basic() {
        assert_eq!(get_sum_absolute_differences(vec![2, 3, 5]), vec![4, 3, 5]);
        assert_eq!(
            get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
