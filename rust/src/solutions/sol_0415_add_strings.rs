#[allow(dead_code)]
pub fn add_strings(num1: String, num2: String) -> String {
    let mut result = Vec::new();
    let mut carry = 0;
    let mut i = num1.len() as i32 - 1;
    let mut j = num2.len() as i32 - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let digit1 = if i >= 0 {
            num1.chars().nth(i as usize).unwrap().to_digit(10).unwrap()
        } else {
            0
        };

        let digit2 = if j >= 0 {
            num2.chars().nth(j as usize).unwrap().to_digit(10).unwrap()
        } else {
            0
        };

        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        result.push((sum % 10).to_string());

        i -= 1;
        j -= 1;
    }

    result.reverse();
    result.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_strings_basic() {
        assert_eq!(add_strings("11".to_string(), "123".to_string()), "134");
        assert_eq!(add_strings("456".to_string(), "77".to_string()), "533");
        assert_eq!(add_strings("0".to_string(), "0".to_string()), "0");
    }
}
