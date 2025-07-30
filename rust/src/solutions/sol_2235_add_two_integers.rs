#![allow(dead_code)]
pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_basic() {
        assert_eq!(sum(12, 5), 17);
        assert_eq!(sum(-10, 4), -6);
    }
}
