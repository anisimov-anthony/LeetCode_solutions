#![allow(dead_code)]
pub fn reverse_only_letters(s: String) -> String {
    if s.is_empty() || s.len() == 1 {
        return s;
    }

    let mut chars: Vec<_> = s.chars().collect();
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if !chars[left].is_alphabetic() {
            left += 1;
            continue;
        }
        if !chars[right].is_alphabetic() {
            right -= 1;
            continue;
        }

        let right_val = chars[right].clone();
        let left_val = chars[left].clone();
        chars[left] = right_val;
        chars[right] = left_val;
        left += 1;
        right -= 1;
    }

    chars.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_only_letters_basic() {
        assert_eq!(
            reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );
        assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
        assert_eq!(
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
