#![allow(dead_code)]
pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let el_sum: i32 = nums.iter().sum();
    let dig_sum: i32 = nums
        .iter()
        .map(|nm| {
            nm.to_string()
                .chars()
                .fold(0, |acc, c| acc + c.to_digit(10).unwrap() as i32)
        })
        .sum();

    el_sum.abs_diff(dig_sum) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_of_sum_basic() {
        assert_eq!(difference_of_sum(vec![1, 15, 6, 3]), 9);
        assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
