#[allow(dead_code)]
pub fn max_vowels(s: String, k: i32) -> i32 {
    let vowels: std::collections::HashSet<char> =
        ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    let chars: Vec<char> = s.chars().collect();
    let k = k as usize;

    let mut current = 0;
    for i in 0..k {
        if vowels.contains(&chars[i]) {
            current += 1;
        }
    }
    let mut max = current;

    for i in k..chars.len() {
        if vowels.contains(&chars[i - k]) {
            current -= 1;
        }
        if vowels.contains(&chars[i]) {
            current += 1;
        }
        max = max.max(current);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels_basic() {
        assert_eq!(max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(max_vowels("aeiou".to_string(), 2), 2);
    }
}
