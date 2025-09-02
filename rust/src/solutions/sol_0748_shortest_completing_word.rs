#![allow(dead_code)]
pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let count_letters = |mut alphabet: [u8; 26], letter| {
        alphabet[letter as usize - 'a' as usize] += 1;
        alphabet
    };
    let freqs = license_plate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold([0; 26], count_letters);

    words
        .into_iter()
        .filter(|w| {
            w.chars()
                .fold([0; 26], count_letters)
                .iter()
                .zip(freqs.iter())
                .all(|(word, plate)| word >= plate)
        })
        .min_by_key(String::len)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_completing_word_basic() {
        assert_eq!(
            shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            ),
            "steps".to_string()
        );
        assert_eq!(
            shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            ),
            "pest".to_string()
        );
    }
}
