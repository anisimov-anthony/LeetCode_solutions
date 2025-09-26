#![allow(dead_code)]
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if sum < k {
        return sum;
    }
    sum % k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_basic() {
        assert_eq!(min_operations(vec![3, 9, 7], 5), 4);
        assert_eq!(min_operations(vec![4, 1, 3], 4), 0);
        assert_eq!(min_operations(vec![3, 2], 6), 5);
    }
}
