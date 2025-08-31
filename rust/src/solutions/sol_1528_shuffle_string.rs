#![allow(dead_code)]
pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let n = s.len();
    let mut res = vec![' '; n];
    for (i, ch) in s.chars().enumerate() {
        res[indices[i] as usize] = ch;
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_string_basic() {
        assert_eq!(
            restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
    }
}
