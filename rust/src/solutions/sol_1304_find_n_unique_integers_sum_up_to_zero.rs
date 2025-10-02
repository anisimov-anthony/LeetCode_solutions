#![allow(dead_code)]
pub fn sum_zero(n: i32) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }

    let mut res = Vec::new();
    if n % 2 == 0 {
        for i in 1..=n / 2 {
            res.push(i);
            res.push(-i);
        }
    } else {
        res.push(-1);
        res.push(1);
        res.push(0);
        for i in 2..=n / 2 {
            res.push(i);
            res.push(-i);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_zero_basic() {
        assert_eq!(sum_zero(5), vec![-1, 1, 0, 2, -2]);
        assert_eq!(sum_zero(2), vec![1, -1]);
        assert_eq!(sum_zero(1), vec![0]);
    }
}
