#![allow(dead_code)]
pub fn count_even(num: i32) -> i32 {
    fn dg_sm(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
    (1..=num)
        .collect::<Vec<i32>>()
        .iter()
        .filter(|nm| dg_sm(**nm) % 2 != 1)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_even_basic() {
        assert_eq!(count_even(4), 2);
        assert_eq!(count_even(30), 14);
    }
}
