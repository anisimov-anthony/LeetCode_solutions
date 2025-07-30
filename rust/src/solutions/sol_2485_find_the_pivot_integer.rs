#![allow(dead_code)]
pub fn pivot_integer(n: i32) -> i32 {
    let all_sum = n * (n + 1) / 2;
    let mut pref_sum = 0;

    for i in 1..=n {
        pref_sum += i;

        if pref_sum == all_sum - pref_sum + i {
            return i;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_integer_basic() {
        assert_eq!(pivot_integer(8), 6);
        assert_eq!(pivot_integer(1), 1);
        assert_eq!(pivot_integer(4), -1);
    }
}
