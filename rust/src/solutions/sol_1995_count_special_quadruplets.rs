#![allow(dead_code)]
pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                for l in k + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == nums[l] {
                        counter += 1;
                    }
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
    fn test_count_quadruplets_basic() {
        assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
        assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
        assert_eq!(count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
    }
}
