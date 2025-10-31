#![allow(dead_code)]
pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut freqs = std::collections::HashMap::new();
    for wd in words.iter() {
        freqs.entry(wd).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut freqs: Vec<_> = freqs.iter().map(|(wd, fr)| (fr, wd)).collect();
    freqs.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    return freqs[0..k as usize]
        .iter()
        .map(|fr| fr.1.to_string())
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_basic() {
        assert_eq!(
            top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            ),
            vec!["i".to_string(), "love".to_string()]
        );
        assert_eq!(
            top_k_frequent(
                vec![
                    "the".to_string(),
                    "day".to_string(),
                    "is".to_string(),
                    "sunny".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "sunny".to_string(),
                    "is".to_string(),
                    "is".to_string()
                ],
                4
            ),
            vec![
                "the".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "day".to_string()
            ]
        );
    }
}
