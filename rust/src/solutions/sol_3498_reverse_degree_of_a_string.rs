#![allow(dead_code)]
pub fn reverse_degree(s: String) -> i32 {
    s.chars().enumerate().fold(0, |acc, (i, ch)| {
        acc + (i as i32 + 1) * (b'z' as i32 - ch as i32 + 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_degree_basic() {
        assert_eq!(reverse_degree("abc".to_string()), 148);
        assert_eq!(reverse_degree("zaza".to_string()), 160);
    }
}
