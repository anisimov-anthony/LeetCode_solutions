#![allow(dead_code)]
pub fn triangular_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    while nums.len() > 1 {
        let mut additive_nums = Vec::new();
        for i in 0..(nums.len() - 1) {
            additive_nums.push((nums[i] + nums[i + 1]) % 10);
        }
        nums = additive_nums;
    }
    nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_sum_basic() {
        assert_eq!(triangular_sum(vec![1, 2, 3, 4, 5]), 8);
        assert_eq!(triangular_sum(vec![5]), 5);
    }
}
