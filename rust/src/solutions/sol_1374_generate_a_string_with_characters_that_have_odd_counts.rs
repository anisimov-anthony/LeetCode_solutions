#![allow(dead_code)]
pub fn generate_the_string(n: i32) -> String {
    let mut res = String::with_capacity(n as usize);
    if n % 2 == 0 {
        res += "a";
        res.push_str(&vec!['b'; n as usize - 1].iter().collect::<String>());
    } else {
        res.push_str(&vec!['a'; n as usize].iter().collect::<String>());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_the_string_basic() {
        assert_eq!(generate_the_string(4), "abbb".to_string());
        assert_eq!(generate_the_string(2), "ab".to_string());
        assert_eq!(generate_the_string(7), "aaaaaaa".to_string());
    }
}
