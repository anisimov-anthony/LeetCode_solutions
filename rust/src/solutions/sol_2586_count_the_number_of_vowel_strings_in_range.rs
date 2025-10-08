#![allow(dead_code)]
pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    words[left as usize..=right as usize]
        .iter()
        .filter(|word| {
            let vowels: std::collections::HashSet<char> =
                std::collections::HashSet::from(['a', 'e', 'i', 'o', 'u']);

            vowels.contains(&word.chars().next().unwrap())
                && vowels.contains(&word.chars().last().unwrap())
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel_strings_basic() {
        assert_eq!(
            vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            ),
            2
        );
        assert_eq!(
            vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            ),
            3
        );
    }
}
