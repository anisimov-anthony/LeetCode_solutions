#![allow(dead_code)]
pub fn count_palindromic_subsequence(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut poses = std::collections::HashMap::new();
    for (i, ch) in s.iter().enumerate() {
        poses.entry(ch).or_insert(Vec::new()).push(i);
    }

    let mut counter = 0;
    for (_, idxs) in poses.iter() {
        if idxs.len() > 1 {
            let mut middle = std::collections::HashSet::new();
            for i in (idxs[0] + 1)..idxs[idxs.len() - 1] {
                middle.insert(s[i]);
            }
            counter += middle.len();
        }
    }

    counter as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_palindromic_subsequence_basic() {
        assert_eq!(count_palindromic_subsequence("aabca".to_string()), 3);
        assert_eq!(count_palindromic_subsequence("adc".to_string()), 0);
        assert_eq!(count_palindromic_subsequence("bbcbaba".to_string()), 4);
    }
}
