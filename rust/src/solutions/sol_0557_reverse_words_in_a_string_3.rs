#[allow(dead_code)]
pub fn reverse_words(s: String) -> String {
    let mut store = Vec::new();
    let words: Vec<&str> = s.split(' ').collect();
    for word in words.iter() {
        store.push(word.chars().rev().collect::<String>())
    }
    store.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words_basic() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(reverse_words("Mr Ding".to_string()), "rM gniD".to_string());
    }
}
