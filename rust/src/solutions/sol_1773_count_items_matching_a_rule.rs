#![allow(dead_code)]
pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut res = 0;

    let index = match rule_key.as_str() {
        "type" => 0,
        "color" => 1,
        _ => 2,
    };

    for item in items.iter() {
        if item[index] == rule_value {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matches_basic() {
        assert_eq!(
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );

        assert_eq!(
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "phone".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
    }
}
