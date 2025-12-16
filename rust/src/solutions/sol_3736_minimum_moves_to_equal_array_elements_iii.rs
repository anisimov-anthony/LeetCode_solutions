#![allow(dead_code)]
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let max_val = *nums.iter().max().unwrap_or(&0);
    let mut result = 0;
    for nm in nums.iter() {
        result += max_val - nm;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves_basic() {
        assert_eq!(min_moves(vec![2, 1, 3]), 3);
        assert_eq!(min_moves(vec![4, 4, 5]), 2);
    }
}
