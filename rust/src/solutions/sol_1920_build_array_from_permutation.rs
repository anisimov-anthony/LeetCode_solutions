#![allow(dead_code)]
pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    for (idx, &var) in nums.iter().enumerate() {
        res[idx as usize] = nums[var as usize];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_array_basic() {
        assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
        assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);
    }
}
