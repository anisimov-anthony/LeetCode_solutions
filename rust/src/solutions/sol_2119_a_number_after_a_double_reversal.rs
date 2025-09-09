#![allow(dead_code)]
pub fn is_same_after_reversals(num: i32) -> bool {
    num == 0 || num % 10 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_after_reversals_basic() {
        assert!(is_same_after_reversals(526));
        assert!(!is_same_after_reversals(1800));
        assert!(is_same_after_reversals(0));
    }
}
