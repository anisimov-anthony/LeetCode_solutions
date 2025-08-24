#![allow(dead_code)]
pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    fn encrypt(num: i32) -> i32 {
        let max_digit = num.to_string().chars().max().unwrap();
        let mut res = String::new();
        for _ in 0..num.to_string().len() {
            res.push(max_digit);
        }
        res.parse::<i32>().unwrap()
    }
    nums.iter().fold(0, |acc, num| acc + encrypt(*num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_encrypted_int_basic() {
        assert_eq!(sum_of_encrypted_int(vec![1, 2, 3]), 6);
        assert_eq!(sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
