#![allow(dead_code)]
pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    nums.iter().enumerate().fold(0, |acc, (i, &val)| {
        acc + nums[0.max(i as i32 - val) as usize..i as usize + 1]
            .iter()
            .sum::<i32>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sum_basic() {
        assert_eq!(subarray_sum(vec![2, 3, 1]), 11);
        assert_eq!(subarray_sum(vec![3, 1, 1, 2]), 13);
    }
}
