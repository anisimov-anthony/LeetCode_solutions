#![allow(dead_code)]
pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n < 3 {
        return 1;
    }

    let mut t1 = 0;
    let mut t2 = 1;
    let mut t3 = 1;

    for _ in 3..=n {
        (t1, t2, t3) = (t2, t3, t1 + t2 + t3);
    }
    t3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci_basic() {
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(25), 1389537);
    }
}
