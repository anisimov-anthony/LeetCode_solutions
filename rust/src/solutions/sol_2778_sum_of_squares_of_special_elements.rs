#![allow(dead_code)]
pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .filter(|ival| nums.len() % (ival.0 + 1) == 0)
        .collect::<Vec<_>>()
        .iter()
        .map(|ival| (*ival.1 as i32).pow(2))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_squares_basic() {
        assert_eq!(sum_of_squares(vec![1, 2, 3, 4]), 21);
        assert_eq!(sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
