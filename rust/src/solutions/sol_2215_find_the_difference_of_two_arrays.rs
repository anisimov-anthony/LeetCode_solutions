#![allow(dead_code)]
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let set1: std::collections::HashSet<i32> = std::collections::HashSet::from_iter(nums1);
    let set2 = std::collections::HashSet::from_iter(nums2);

    let mut df1 = set1.difference(&set2).copied().collect::<Vec<i32>>();
    let mut df2 = set2.difference(&set1).copied().collect::<Vec<i32>>();

    df1.sort();
    df2.sort();

    vec![df1, df2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_difference_basic() {
        assert_eq!(
            find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            vec![vec![1, 3], vec![4, 6]]
        );
        assert_eq!(
            find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }
}
