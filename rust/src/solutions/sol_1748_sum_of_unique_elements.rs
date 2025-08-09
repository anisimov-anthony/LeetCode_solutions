#![allow(dead_code)]
pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut frs = std::collections::HashMap::new();
    for num in nums.iter() {
        frs.entry(num).and_modify(|fr| *fr += 1).or_insert(1);
    }
    nums.iter().filter(|nm| *frs.get(nm).unwrap() == 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_unique_basic() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
