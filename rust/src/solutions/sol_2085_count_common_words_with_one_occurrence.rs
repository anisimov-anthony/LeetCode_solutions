#![allow(dead_code)]
pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut w1_occs = std::collections::HashMap::new();
    for wd in words1.iter() {
        w1_occs.entry(wd).and_modify(|fr| *fr += 1).or_insert(1);
    }
    let mut w2_occs = std::collections::HashMap::new();
    for wd in words2.iter() {
        w2_occs.entry(wd).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut counter = 0;
    for (wd, &fr) in w1_occs.iter() {
        if fr == 1 {
            if let Some(&fr2) = w2_occs.get(wd) {
                if fr2 == 1 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_basic() {
        assert_eq!(
            count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            ),
            0
        );
        assert_eq!(
            count_words(
                vec!["a".to_string(), "ab".to_string()],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            ),
            1
        );
    }
}
