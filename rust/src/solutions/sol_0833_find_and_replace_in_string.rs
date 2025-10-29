#![allow(dead_code)]
pub fn find_replace_string(
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut ops: Vec<(usize, String, String)> = indices
        .into_iter()
        .zip(sources.into_iter())
        .zip(targets.into_iter())
        .map(|((idx, src), tgt)| (idx as usize, src, tgt))
        .collect();

    ops.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let s_chars: Vec<char> = s.chars().collect();
    let mut result = s_chars.clone();

    for (idx, source, target) in ops {
        let src_chars: Vec<char> = source.chars().collect();

        if idx + src_chars.len() <= s_chars.len()
            && s_chars[idx..idx + src_chars.len()] == src_chars[..]
        {
            result.splice(idx..idx + src_chars.len(), target.chars());
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_replace_string_basic() {
        assert_eq!(
            find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["a".to_string(), "cd".to_string()],
                vec!["eee".to_string(), "ffff".to_string()]
            ),
            "eeebffff"
        );
        assert_eq!(
            find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["ab".to_string(), "ec".to_string()],
                vec!["eee".to_string(), "ffff".to_string()]
            ),
            "eeecd"
        )
    }
}
