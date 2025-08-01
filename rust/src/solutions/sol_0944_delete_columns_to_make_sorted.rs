#![allow(dead_code)]
pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    if strs.is_empty() {
        return 0;
    }

    let mut deletions = 0;
    let n = strs[0].len();

    for col in 0..n {
        let mut prev_char = None;
        for s in &strs {
            let current_char = s.chars().nth(col).unwrap();
            if let Some(prev) = prev_char {
                if current_char < prev {
                    deletions += 1;
                    break;
                }
            }
            prev_char = Some(current_char);
        }
    }

    deletions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_deletion_size_basic() {
        assert_eq!(
            min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
        assert_eq!(min_deletion_size(vec!["a".to_string(), "b".to_string()]), 0);
        assert_eq!(
            min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
    }
}
