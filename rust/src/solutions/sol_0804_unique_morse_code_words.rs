#![allow(dead_code)]
pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let alphabet = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    let mut uniqs = std::collections::HashSet::new();
    for word in words.iter() {
        let mut encoded = String::new();
        for ch in word.chars() {
            let index = (ch.to_ascii_lowercase() as u8 - b'a') as usize;
            encoded.push_str(alphabet[index]);
        }

        uniqs.insert(encoded);
    }

    uniqs.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_morse_representations_basic() {
        assert_eq!(
            unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );
        assert_eq!(unique_morse_representations(vec!["a".to_string()]), 1);
    }
}
