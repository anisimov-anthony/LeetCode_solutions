#[allow(dead_code)]
pub fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_case_basic() {
        assert_eq!(to_lower_case("Hello".to_string()), "hello".to_string());
        assert_eq!(to_lower_case("here".to_string()), "here".to_string());
        assert_eq!(to_lower_case("LOVELY".to_string()), "lovely".to_string());
    }
}
