#![allow(dead_code)]
pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let diff = max - min - 2 * k;
    diff.max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_range_i_basic() {
        assert_eq!(smallest_range_i(vec![1], 0), 0);
        assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
