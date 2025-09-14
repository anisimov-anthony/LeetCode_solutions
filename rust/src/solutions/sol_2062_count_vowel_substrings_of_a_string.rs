#![allow(dead_code)]
pub fn count_vowel_substrings(word: String) -> i32 {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let chars: Vec<_> = word.chars().collect();
    let mut res = 0;
    for i in 0..chars.len() {
        if !vowels.contains(&chars[i]) {
            continue;
        }

        let mut substr_set = std::collections::HashSet::new();
        for j in i..chars.len() {
            if !vowels.contains(&chars[j]) {
                break;
            }

            substr_set.insert(chars[j]);
            if substr_set.len() == 5 {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowel_substrings_basic() {
        assert_eq!(count_vowel_substrings("aeiouu".to_string()), 2);
        assert_eq!(count_vowel_substrings("unicornarihan".to_string()), 0);
        assert_eq!(count_vowel_substrings("cuaieuouac".to_string()), 7);
    }
}
