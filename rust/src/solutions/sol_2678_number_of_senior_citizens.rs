#![allow(dead_code)]
pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .map(|dt| {
            dt.chars().nth(dt.len() - 4).unwrap().to_digit(10).unwrap() * 10
                + dt.chars().nth(dt.len() - 3).unwrap().to_digit(10).unwrap()
        })
        .filter(|age| *age > 60)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_seniors_basic() {
        assert_eq!(
            count_seniors(vec![
                "7868190130M7522".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ]),
            2
        );
        assert_eq!(
            count_seniors(vec![
                "1313579440F2036".to_string(),
                "2921522980M5644".to_string()
            ]),
            0
        );
    }
}
