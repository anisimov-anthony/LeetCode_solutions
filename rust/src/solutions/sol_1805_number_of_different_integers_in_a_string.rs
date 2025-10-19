#![allow(dead_code)]
pub fn num_different_integers(word: String) -> i32 {
    fn convert_num(s: String) -> String {
        if s.chars().all(|ch| ch == '0') {
            return "0".to_string();
        }
        let mut converted = String::new();
        let mut start_added = false;
        for ch in s.chars() {
            if ch != '0' {
                start_added = true;
            }
            if start_added {
                converted.push(ch);
            }
        }

        converted
    }

    let mut integers = std::collections::HashSet::new();
    let mut buffer = String::new();
    for ch in word.chars() {
        if !ch.is_ascii_alphabetic() {
            buffer.push(ch);
        } else {
            if !buffer.is_empty() {
                integers.insert(convert_num(buffer.clone()));
                buffer.clear();
            }
        }
    }
    if !buffer.is_empty() {
        integers.insert(convert_num(buffer.clone()));
        buffer.clear();
    }

    integers.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_different_integers_basic() {
        assert_eq!(num_different_integers("a123bc34d8ef34".to_string()), 3);
        assert_eq!(num_different_integers("leet1234code234".to_string()), 2);
        assert_eq!(num_different_integers("a1b01c001".to_string()), 1);
    }
}
