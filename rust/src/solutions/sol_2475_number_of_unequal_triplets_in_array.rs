#![allow(dead_code)]
pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if (nums[i] != nums[j]) && (nums[j] != nums[k]) && (nums[i] != nums[k]) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unequal_triplets_basic() {
        assert_eq!(unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
        assert_eq!(unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }
}
