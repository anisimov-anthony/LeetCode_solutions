#![allow(dead_code)]
pub fn is_circular_sentence(sentence: String) -> bool {
    let mut starts_ends = Vec::new();
    for word in sentence.split_whitespace() {
        starts_ends.push(word.chars().nth(0).unwrap());
        starts_ends.push(word.chars().last().unwrap());
    }
    starts_ends.push(starts_ends[0]);
    starts_ends.remove(0);
    starts_ends.chunks(2).all(|wnd| wnd[0] == wnd[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_circular_sentence_basic() {
        assert!(is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
        assert!(is_circular_sentence("eetcode".to_string()));
        assert!(!is_circular_sentence("Leetcode is cool".to_string()));
    }
}
