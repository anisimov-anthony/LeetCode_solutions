#![allow(dead_code)]
pub fn digit_sum(s: String, k: i32) -> String {
    let mut s = s;

    while s.len() > k as usize {
        let mut next = String::new();

        for chunk in s.as_bytes().chunks(k as usize) {
            let sum: u32 = chunk.iter().map(|&b| (b - b'0') as u32).sum();
            next.push_str(&sum.to_string());
        }

        s = next;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum_basic() {
        assert_eq!(digit_sum("11111222223".to_string(), 3), "135");
        assert_eq!(digit_sum("00000000".to_string(), 3), "000");
    }
}
