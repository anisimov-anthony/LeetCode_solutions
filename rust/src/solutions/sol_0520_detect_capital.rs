#![allow(dead_code)]
pub fn detect_capital_use(word: String) -> bool {
    let mut third_cond = true;
    let mut tmp = true;
    for (i, ch) in word.chars().enumerate() {
        if i == 0 {
            if ch.is_uppercase() {
                tmp = true;
                third_cond = true;
            }
        } else {
            if tmp == true && ch.is_uppercase() {
                third_cond = false;
            }
        }
    }
    word.chars().all(|ch| ch.is_lowercase())
        || word.chars().all(|ch| ch.is_uppercase())
        || third_cond
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_capital_use_basic() {
        assert!(detect_capital_use("USA".to_string()));
        assert!(!detect_capital_use("FlaG".to_string()));
    }
}
