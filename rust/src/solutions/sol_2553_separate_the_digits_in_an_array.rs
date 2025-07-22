#![allow(dead_code)]
pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for num in nums.iter() {
        let dgs: Vec<_> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        for &dg in dgs.iter() {
            result.push(dg as i32);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_digits_basic() {
        assert_eq!(
            separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
        assert_eq!(separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    }
}
