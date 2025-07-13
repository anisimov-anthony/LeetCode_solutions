#[allow(dead_code)]
pub fn truncate_sentence(s: String, k: i32) -> String {
    let mut collection_snts = s.split_whitespace().collect::<Vec<&str>>();
    collection_snts.truncate(k as usize);

    let mut result = String::new();
    for st in collection_snts.iter() {
        result.push_str(st);
        result.push_str(" ");
    }

    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_sentence_basic() {
        assert_eq!(
            truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you".to_string()
        );
        assert_eq!(
            truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution".to_string()
        );
        assert_eq!(
            truncate_sentence("chopper is not a tanuki".to_string(), 5),
            "chopper is not a tanuki".to_string()
        );
    }
}
