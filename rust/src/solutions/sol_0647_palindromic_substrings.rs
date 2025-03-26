#[allow(dead_code)]
pub fn count_substrings(s: String) -> i32 {
    let mut result = 0;
    let bytes = s.as_bytes();

    for i in 0..s.len() {
        result += count_palindromes(bytes, i, i);
        result += count_palindromes(bytes, i, i + 1);
    }

    result
}

fn count_palindromes(bytes: &[u8], mut left: usize, mut right: usize) -> i32 {
    let mut count = 0;
    while left <= right && right < bytes.len() && bytes[left] == bytes[right] {
        count += 1;
        if left == 0 {
            break;
        }
        left -= 1;
        right += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_substrings_basic() {
        assert_eq!(count_substrings("abc".to_string()), 3);
        assert_eq!(count_substrings("aaa".to_string()), 6);
    }
}
