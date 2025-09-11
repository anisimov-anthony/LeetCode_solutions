#![allow(dead_code)]
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums1.len());
    for nm1 in nums1.iter() {
        if let Some(idx) = nums2.iter().enumerate().find(|&x| x.1 == nm1) {
            if idx.0 < nums2.len() {
                if let Some(next) = nums2
                    .get(idx.0..)
                    .unwrap()
                    .iter()
                    .find(|item| *item > idx.1)
                {
                    res.push(*next);
                } else {
                    res.push(-1);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element_basic() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
