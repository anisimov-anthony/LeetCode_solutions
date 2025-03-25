#[allow(dead_code)]
pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let sum1 = coordinate1.as_bytes()[0] as i32 + coordinate1.as_bytes()[1] as i32;
    let sum2 = coordinate2.as_bytes()[0] as i32 + coordinate2.as_bytes()[1] as i32;
    sum1 % 2 == sum2 % 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_two_chessboards_basic() {
        assert_eq!(
            check_two_chessboards("a1".to_string(), "c3".to_string()),
            true
        );
        assert_eq!(
            check_two_chessboards("a1".to_string(), "h3".to_string()),
            false
        );
    }
}
