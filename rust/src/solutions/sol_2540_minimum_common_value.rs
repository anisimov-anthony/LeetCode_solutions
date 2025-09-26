#![allow(dead_code)]
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums2: std::collections::HashSet<_> = nums2.into_iter().collect();
    for &i in nums1.iter() {
        if nums2.contains(&i) {
            return i;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common_basic() {
        assert_eq!(get_common(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
