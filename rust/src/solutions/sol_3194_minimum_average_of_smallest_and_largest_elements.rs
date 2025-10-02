#![allow(dead_code)]
pub fn minimum_average(nums: Vec<i32>) -> f64 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = f64::MAX;

    for i in 0..nums.len() / 2 {
        let min = nums[i];
        let max = nums[nums.len() - 1 - i];

        let avg = (min + max) as f64 / 2.0;
        res = res.min(avg);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_average_basic() {
        assert_eq!(minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]), 5.5_f64);
        assert_eq!(minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5_f64);
        assert_eq!(minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0_f64);
    }
}
