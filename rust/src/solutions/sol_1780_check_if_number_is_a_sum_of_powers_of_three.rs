#![allow(dead_code)]
pub fn check_powers_of_three(n: i32) -> bool {
    (0..15).all(|i| (n / 3_i32.pow(i)) % 3 != 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_powers_of_three_basic() {
        assert!(check_powers_of_three(12));
        assert!(check_powers_of_three(91));
        assert!(!check_powers_of_three(21));
    }
}
