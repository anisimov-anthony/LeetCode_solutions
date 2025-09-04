#![allow(dead_code)]
pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut result = Vec::new();
    let mut stream: Vec<_> = (1..=n).rev().collect();
    for tg in target.iter() {
        while stream.iter().last().unwrap() != tg {
            result.push("Push".to_string());
            stream.pop();
            result.push("Pop".to_string());
        }
        stream.pop();
        result.push("Push".to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_array_basic() {
        assert_eq!(
            build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
        assert_eq!(
            build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );

        assert_eq!(
            build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string()]
        );
    }
}
