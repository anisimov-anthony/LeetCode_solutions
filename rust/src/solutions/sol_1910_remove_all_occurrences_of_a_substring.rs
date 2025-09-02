#![allow(dead_code)]
pub fn remove_occurrences(s: String, part: String) -> String {
    let mut s = s;
    while let Some(first_occurence) = s.find(&part) {
        s.replace_range(first_occurence..first_occurence + part.len(), "");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_occurrences_basic() {
        assert_eq!(
            remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
            "dab".to_string()
        );
        assert_eq!(
            remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
            "ab".to_string()
        );
    }
}
