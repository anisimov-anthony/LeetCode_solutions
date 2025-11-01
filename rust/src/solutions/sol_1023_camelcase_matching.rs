#![allow(dead_code)]
pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let mut result = Vec::with_capacity(queries.len());
    let pattern: Vec<_> = pattern.chars().collect();

    for query in queries.iter() {
        let mut matched = true;
        let mut second = 0;
        let query: Vec<_> = query.chars().collect();

        for &ch in query.iter() {
            if second < pattern.len() && ch == pattern[second] {
                second += 1;
            } else if ch.is_uppercase() {
                matched = false;
                break;
            }
        }

        result.push(matched && second == pattern.len());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_match_basic() {
        assert_eq!(
            camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FB".to_string()
            ),
            vec![true, false, true, true, false]
        );
        assert_eq!(
            camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FoBa".to_string()
            ),
            vec![true, false, true, false, false]
        );
        assert_eq!(
            camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FoBaT".to_string()
            ),
            vec![false, true, false, false, false]
        );
    }
}
