#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> i32 {
    // corner case
    if s.is_empty() || s.len() == 1 {
        return s.len().try_into().unwrap();
    }

    // main case
    let mut freqs = std::collections::HashMap::new();

    for ch in s.chars() {
        freqs
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let even = freqs
        .iter()
        .filter(|fr| fr.1 % 2 == 0)
        .fold(0, |acc, fl| acc + fl.1);

    let mut has_odd = false;
    for (_, fr) in freqs.iter() {
        if fr % 2 != 0 {
            has_odd = true;
            break;
        }
    }

    let odd_minus_one = freqs
        .iter()
        .filter(|fr| fr.1 % 2 != 0)
        .fold(0, |acc, fl| acc + fl.1 - 1);

    if has_odd {
        return even + odd_minus_one + 1;
    }

    even
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_basic() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }
}
