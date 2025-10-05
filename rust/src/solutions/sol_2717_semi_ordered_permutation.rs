#![allow(dead_code)]
pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let one_idx = nums
        .iter()
        .enumerate()
        .find(|(_, val)| **val == 1)
        .map(|en| en.0)
        .unwrap();

    let n_idx = nums
        .iter()
        .enumerate()
        .find(|(_, val)| **val == nums.len() as i32)
        .map(|en| en.0)
        .unwrap();
    if one_idx < n_idx {
        return one_idx as i32 + (nums.len() as i32 - n_idx as i32 - 1);
    }
    one_idx as i32 + (nums.len() as i32 - n_idx as i32 - 1) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semi_ordered_permutation_nasic() {
        assert_eq!(semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
        assert_eq!(semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
