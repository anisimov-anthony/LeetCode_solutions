#![allow(dead_code)]
pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut merged = Vec::with_capacity(left.len() + right.len());
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                merged.push(left[i]);
                i += 1;
            } else {
                merged.push(right[j]);
                j += 1;
            }
        }

        merged.extend_from_slice(&left[i..]);
        merged.extend_from_slice(&right[j..]);
        merged
    }

    if nums.len() <= 1 {
        return nums;
    }

    let mid = nums.len() / 2;
    let left_half = sort_array(nums[..mid].to_vec());
    let right_half = sort_array(nums[mid..].to_vec());

    merge(&left_half, &right_half)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_basic() {
        assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
    }
}
