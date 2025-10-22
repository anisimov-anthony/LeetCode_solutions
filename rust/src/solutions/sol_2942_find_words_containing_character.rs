#![allow(dead_code)]
pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
        .iter()
        .enumerate()
        .filter(|(_, word)| word.contains(x))
        .map(|(i, _)| i as i32)
        .fold(Vec::new(), |mut acc, fl| {
            acc.push(fl);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words_containing_basic() {
        assert_eq!(
            find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            vec![0, 1]
        );
        assert_eq!(
            find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'a'
            ),
            vec![0, 2]
        );
        assert_eq!(
            find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'z'
            ),
            Vec::new()
        );
    }
}
