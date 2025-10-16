#![allow(dead_code)]
pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums.into_iter()
        .fold((0, None), |(n, y), x| match y {
            Some(s) if x - s <= k => (n, y),
            _ => (n + 1, Some(x)),
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_array_basic() {
        assert_eq!(partition_array(vec![3, 6, 1, 2, 5], 2), 2);
        assert_eq!(partition_array(vec![1, 2, 3], 1), 2);
        assert_eq!(partition_array(vec![2, 2, 4, 5], 0), 3);
    }
}
