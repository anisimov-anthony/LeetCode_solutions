#![allow(dead_code)]
pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut num1 = 0;
    let mut num2 = 0;
    (1..=n as usize).for_each(|nm| {
        if nm as i32 % m != 0 {
            num1 += nm;
        } else {
            num2 += nm;
        }
    });

    num1 as i32 - num2 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_of_sums_basic() {
        assert_eq!(difference_of_sums(10, 3), 19);
        assert_eq!(difference_of_sums(5, 6), 15);
        assert_eq!(difference_of_sums(5, 1), -15);
    }
}
