#[allow(dead_code)]
pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let fs: std::collections::HashSet<_> = nums1.iter().collect();
    let ss: std::collections::HashSet<_> = nums2.iter().collect();

    let count1 = nums1.iter().filter(|&x| ss.contains(x)).count() as i32;
    let count2 = nums2.iter().filter(|&x| fs.contains(x)).count() as i32;

    vec![count1, count2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersection_values_basic() {
        assert_eq!(
            find_intersection_values(vec![2, 3, 2], vec![1, 2]),
            vec![2, 1]
        );
        assert_eq!(
            find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            vec![3, 4]
        );
        assert_eq!(
            find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
            vec![0, 0]
        );
    }
}
