#![allow(dead_code)]
pub fn common_chars(words: Vec<String>) -> Vec<String> {
    if words.is_empty() {
        return Vec::new();
    }

    let mut common_counts: std::collections::HashMap<char, usize> =
        words[0]
            .chars()
            .fold(std::collections::HashMap::new(), |mut map, c| {
                *map.entry(c).or_default() += 1;
                map
            });

    for word in words.iter().skip(1) {
        let current_counts = word
            .chars()
            .fold(std::collections::HashMap::new(), |mut map, c| {
                *map.entry(c).or_default() += 1;
                map
            });

        common_counts.retain(|c, count| {
            if let Some(&current) = current_counts.get(c) {
                *count = (*count).min(current);
                true
            } else {
                false
            }
        });
    }

    let mut result = Vec::new();
    for (c, &count) in common_counts.iter() {
        for _ in 0..count {
            result.push(c.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_chars_basic() {
        assert_eq!(
            common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );

        assert_eq!(
            common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            vec!["c".to_string(), "o".to_string()]
        );
    }
}
