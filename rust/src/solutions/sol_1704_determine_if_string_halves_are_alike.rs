#![allow(dead_code)]
pub fn halves_are_alike(s: String) -> bool {
    let s: Vec<_> = s.chars().collect();
    let mut left = 0;
    let mut right = s.len() - 1;
    let alikes: std::collections::HashSet<_> = std::collections::HashSet::from_iter(
        vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].into_iter(),
    );
    let mut la = 0;
    let mut ra = 0;
    while left < right {
        if alikes.contains(&s[left]) {
            la += 1;
        }
        if alikes.contains(&s[right]) {
            ra += 1;
        }
        left += 1;
        right -= 1;
    }

    la == ra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halves_are_alike_basic() {
        assert!(halves_are_alike("book".to_string()));
        assert!(!halves_are_alike("textbook".to_string()));
    }
}
