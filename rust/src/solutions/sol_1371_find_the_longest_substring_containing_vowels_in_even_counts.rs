#![allow(dead_code)]
pub fn find_the_longest_substring(s: String) -> i32 {
    let mut result = 0;
    let mut bit_mask = 0;
    let mut masks = std::collections::HashMap::new();
    masks.insert(0, -1);

    for (i, ch) in s.bytes().enumerate() {
        match ch {
            b'a' => bit_mask ^= 1 << 0,
            b'e' => bit_mask ^= 1 << 1,
            b'i' => bit_mask ^= 1 << 2,
            b'o' => bit_mask ^= 1 << 3,
            b'u' => bit_mask ^= 1 << 4,
            _ => {}
        }

        if let Some(prev_index) = masks.get(&bit_mask) {
            result = result.max(i as i32 - prev_index);
        } else {
            masks.insert(bit_mask, i as i32);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_longest_substring_basic() {
        assert_eq!(
            find_the_longest_substring("eleetminicoworoep".to_string()),
            13
        );
        assert_eq!(find_the_longest_substring("leetcodeisgreat".to_string()), 5);
        assert_eq!(find_the_longest_substring("bcbcbc".to_string()), 6);
    }
}
