#![allow(dead_code)]
pub fn replace_digits(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    for i in (1..chars.len()).step_by(2) {
        let shift = chars[i] as u8 - b'0';
        chars[i] = ((chars[i - 1] as u8) + shift) as char;
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_digits_basic() {
        assert_eq!(replace_digits("a1c1e1".to_string()), "abcdef".to_string());
        assert_eq!(
            replace_digits("a1b2c3d4e".to_string()),
            "abbdcfdhe".to_string()
        );
    }
}
