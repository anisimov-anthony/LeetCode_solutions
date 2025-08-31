#![allow(dead_code)]
pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut chars_count = std::collections::HashMap::new();
    for c in chars.chars() {
        *chars_count.entry(c).or_insert(0) += 1;
    }

    let can_form = |word: &String| -> bool {
        let mut word_count = std::collections::HashMap::new();
        for c in word.chars() {
            *word_count.entry(c).or_insert(0) += 1;
            if word_count[&c] > *chars_count.get(&c).unwrap_or(&0) {
                return false;
            }
        }
        true
    };

    words
        .iter()
        .filter(|wd| can_form(wd))
        .fold(0, |acc, filtered_word| acc + filtered_word.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_characters_basic() {
        assert_eq!(
            count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );

        assert_eq!(
            count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
