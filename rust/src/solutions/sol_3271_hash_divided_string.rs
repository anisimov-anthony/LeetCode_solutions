#![allow(dead_code)]
pub fn string_hash(s: String, k: i32) -> String {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(k as usize)
        .map(|chunk| {
            let sum: i32 = chunk.iter().map(|&ch| (ch as u8 - b'a') as i32).sum();
            ((sum % 26) as u8 + b'a') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hash_basic() {
        assert_eq!(string_hash("abcd".to_string(), 2), "bf");
        assert_eq!(string_hash("mxz".to_string(), 3), "i");
    }
}
