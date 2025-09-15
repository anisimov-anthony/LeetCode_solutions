#![allow(dead_code)]
pub fn maximum_value(strs: Vec<String>) -> i32 {
    fn max_vl(s: String) -> i32 {
        if s.chars().all(|ch| ch.is_numeric()) {
            return s.parse::<i32>().unwrap();
        }
        return s.len() as i32;
    }
    strs.iter()
        .map(|s| max_vl(s.to_string()))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_value_basic() {
        assert_eq!(
            maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]),
            5
        );
        assert_eq!(
            maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ]),
            1
        );
    }
}
