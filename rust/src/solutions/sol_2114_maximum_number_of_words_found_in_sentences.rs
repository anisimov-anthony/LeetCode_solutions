#![allow(dead_code)]
pub fn most_words_found(sentences: Vec<String>) -> i32 {
    *sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count())
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_words_found_basic() {
        assert_eq!(
            most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ]),
            6
        );
        assert_eq!(
            most_words_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string()
            ]),
            3
        );
    }
}
