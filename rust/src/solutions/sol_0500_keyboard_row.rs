#![allow(dead_code)]
pub fn find_words(words: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let rows = vec![
        "qwertyuiop".to_string(),
        "asdfghjkl".to_string(),
        "zxcvbnm".to_string(),
    ];

    for word in words.iter() {
        let mut row_idx = 0;
        for i in 0..rows.len() {
            if rows[i].contains(word.to_lowercase().chars().nth(0).unwrap()) {
                row_idx = i;
                break;
            }
        }
        let mut added = true;

        for smb in word.chars() {
            if !rows[row_idx].contains(&smb.to_string().to_lowercase()) {
                added = false;
            }
        }

        if added {
            res.push(word.to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words_basic() {
        assert_eq!(
            find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ]),
            vec!["Alaska", "Dad"]
        );
        assert_eq!(find_words(vec!["omk".to_string()]), Vec::<String>::new());
        assert_eq!(
            find_words(vec!["adsdf".to_string(), "sfd".to_string()]),
            vec!["adsdf".to_string(), "sfd".to_string()]
        )
    }
}
