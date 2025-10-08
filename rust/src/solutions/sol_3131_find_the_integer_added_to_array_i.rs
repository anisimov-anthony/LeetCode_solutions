#![allow(dead_code)]
pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums2.iter().max().unwrap() - nums1.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_added_integer_basic() {
        assert_eq!(added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
        assert_eq!(added_integer(vec![10], vec![5]), -5);
        assert_eq!(added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 0);
    }
}
