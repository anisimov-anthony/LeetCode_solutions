#![allow(dead_code)]
pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(&a));
    nums.dedup();

    nums[..(k as usize).min(nums.len())].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_k_distinct_basic() {
        assert_eq!(
            max_k_distinct(vec![84, 93, 100, 77, 90], 3),
            vec![100, 93, 90]
        );
        assert_eq!(
            max_k_distinct(vec![84, 93, 100, 77, 93], 3),
            vec![100, 93, 84]
        );
        assert_eq!(max_k_distinct(vec![1, 1, 1, 2, 2, 2], 6), vec![2, 1]);
    }
}
