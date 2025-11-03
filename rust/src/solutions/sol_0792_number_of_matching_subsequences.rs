#![allow(dead_code)]
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let s_chars: Vec<_> = s.chars().collect();
    let mut positions: std::collections::HashMap<char, Vec<usize>> =
        std::collections::HashMap::new();

    for (i, &ch) in s_chars.iter().enumerate() {
        positions.entry(ch).or_default().push(i);
    }

    let mut result = 0;

    for word in words {
        let mut ptr: isize = -1;
        let mut matched = true;

        for c in word.chars() {
            if let Some(v) = positions.get(&c) {
                let target = (ptr + 1).max(0) as usize;

                match v.binary_search(&target) {
                    Ok(i) => ptr = v[i] as isize,
                    Err(i) if i < v.len() => ptr = v[i] as isize,
                    _ => {
                        matched = false;
                        break;
                    }
                }
            } else {
                matched = false;
                break;
            }
        }

        if matched {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_matching_subseq_basic() {
        assert_eq!(
            num_matching_subseq(
                "abcde".to_string(),
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "acd".to_string(),
                    "ace".to_string()
                ]
            ),
            3
        );
        assert_eq!(
            num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec![
                    "ahjpjau".to_string(),
                    "ja".to_string(),
                    "ahbwzgqnuk".to_string(),
                    "tnmlanowax".to_string()
                ]
            ),
            2
        );
    }
}
