#![allow(dead_code)]
pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();

    nums.iter()
        .enumerate()
        .filter(|(_, val)| **val == target)
        .map(|fl| fl.0 as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_indices_basic() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
