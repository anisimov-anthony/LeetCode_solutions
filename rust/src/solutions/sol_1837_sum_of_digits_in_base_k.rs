#![allow(dead_code)]
pub fn sum_base(n: i32, k: i32) -> i32 {
    let mut n = n;
    let mut res = 0;
    while n > 0 {
        res += n % k;
        n = n / k;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_base_basic() {
        assert_eq!(sum_base(34, 6), 9);
        assert_eq!(sum_base(10, 10), 1);
    }
}
