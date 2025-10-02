#![allow(dead_code)]
pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut poses = std::collections::HashMap::new();
    for (i, ch) in s.chars().enumerate() {
        poses.entry(ch).or_insert(Vec::new()).push(i as i32);
    }
    for (i, ch) in t.chars().enumerate() {
        poses.entry(ch).or_insert(Vec::new()).push(i as i32);
    }
    poses
        .iter()
        .fold(0, |acc, ps| acc + ps.1[0].abs_diff(ps.1[1])) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_permutation_difference_basic() {
        assert_eq!(
            find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
        assert_eq!(
            find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
