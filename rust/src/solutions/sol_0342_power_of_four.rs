#![allow(dead_code)]
pub fn is_power_of_four(n: i32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four_basic() {
        assert!(is_power_of_four(16));
        assert!(!is_power_of_four(5));
        assert!(is_power_of_four(1));
    }
}
