#![allow(dead_code)]
pub fn repeated_character(s: String) -> char {
    let mut freqs = std::collections::HashMap::new();
    for ch in s.chars() {
        freqs.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
        if let Some(fr) = freqs.get(&ch) {
            if *fr > 1 {
                return ch;
            }
        }
    }
    '0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_character_basic() {
        assert_eq!(repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(repeated_character("abcdd".to_string()), 'd');
    }
}
