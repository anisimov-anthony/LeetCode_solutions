#![allow(dead_code)]
pub fn smallest_index(nums: Vec<i32>) -> i32 {
    fn sum_of_digits(nm: i32) -> i32 {
        nm.to_string()
            .chars()
            .fold(0, |acc, ch| acc + ch.to_digit(10).unwrap()) as i32
    }
    if let Some(min) = nums
        .iter()
        .enumerate()
        .filter(|pr| sum_of_digits(*pr.1) == pr.0 as i32)
        .map(|fl| fl.0)
        .min()
    {
        return min as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_index_basic() {
        assert_eq!(smallest_index(vec![1, 3, 2]), 2);
        assert_eq!(smallest_index(vec![1, 10, 11]), 1);
        assert_eq!(smallest_index(vec![1, 2, 3]), -1);
    }
}
