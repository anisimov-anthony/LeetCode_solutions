#[allow(dead_code)]
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (i < j) && nums[i] == nums[j] {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs_basic() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }
}
