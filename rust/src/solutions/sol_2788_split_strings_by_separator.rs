#![allow(dead_code)]
pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .iter()
        .flat_map(|wds| wds.split(separator).map(String::from))
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words_by_separator_basic() {
        assert_eq!(
            split_words_by_separator(
                vec![
                    "one.two.three".to_string(),
                    "four.five".to_string(),
                    "six".to_string()
                ],
                '.'
            ),
            vec!["one", "two", "three", "four", "five", "six"]
        );
        assert_eq!(
            split_words_by_separator(vec!["$easy$".to_string(), "$problem$".to_string()], '$'),
            vec!["easy", "problem"]
        );
        assert_eq!(
            split_words_by_separator(vec!["|||".to_string()], '|'),
            Vec::<String>::new()
        )
    }
}
