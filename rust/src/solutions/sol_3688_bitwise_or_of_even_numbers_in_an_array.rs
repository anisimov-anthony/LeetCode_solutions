#![allow(dead_code)]
pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
    nums.iter()
        .filter(|&nm| nm % 2 == 0)
        .fold(0, |acc, fl_nm| acc | fl_nm) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_number_bitwise_o_rs_basic() {
        assert_eq!(even_number_bitwise_o_rs(vec![1, 2, 3, 4, 5, 6]), 6);
        assert_eq!(even_number_bitwise_o_rs(vec![7, 9, 11]), 0);
        assert_eq!(even_number_bitwise_o_rs(vec![1, 8, 16]), 24);
    }
}
