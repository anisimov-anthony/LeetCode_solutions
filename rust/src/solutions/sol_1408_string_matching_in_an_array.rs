#![allow(dead_code)]
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[j].contains(&words[i]) && !res.contains(&words[i]) {
                res.push(words[i].clone());
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_matching_basic() {
        assert_eq!(
            string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string()
            ]),
            vec!["as", "hero"]
        );
        assert_eq!(
            string_matching(vec![
                "leetcode".to_string(),
                "et".to_string(),
                "code".to_string()
            ]),
            vec!["et", "code"]
        );
        assert_eq!(
            string_matching(vec![
                "blue".to_string(),
                "green".to_string(),
                "bu".to_string()
            ]),
            Vec::<String>::new()
        );
    }
}
