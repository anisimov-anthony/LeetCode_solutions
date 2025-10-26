#![allow(dead_code)]
pub fn thousand_separator(n: i32) -> String {
    let digits: Vec<char> = n.to_string().chars().collect();
    let len = digits.len();
    if len == 0 {
        return String::new();
    }

    let mut result = Vec::new();
    for (i, digit) in digits.iter().enumerate() {
        result.push(*digit);
        if i < len - 1 && (len - i - 1) % 3 == 0 {
            result.push('.');
        }
    }

    result.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thousand_separator_basic() {
        assert_eq!(thousand_separator(987), "987");
        assert_eq!(thousand_separator(1234), "1.234")
    }
}
