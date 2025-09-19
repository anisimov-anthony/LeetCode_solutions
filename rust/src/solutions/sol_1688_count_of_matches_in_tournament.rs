#![allow(dead_code)]
pub fn number_of_matches(n: i32) -> i32 {
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_matches_basic() {
        assert_eq!(number_of_matches(7), 6);
        assert_eq!(number_of_matches(14), 13);
    }
}
