#![allow(dead_code)]
pub fn merge_alternately(word1: String, word2: String) -> String {
    let word1: Vec<_> = word1.chars().collect();
    let word2: Vec<_> = word2.chars().collect();
    let mut res = Vec::with_capacity(word1.len() + word2.len());

    let mut first = 0;
    let mut second = 0;

    while first < word1.len() || second < word2.len() {
        if first < word1.len() {
            res.push(word1[first]);
            first += 1;
        }
        if second < word2.len() {
            res.push(word2[second]);
            second += 1;
        }
    }

    res.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately_basic() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
