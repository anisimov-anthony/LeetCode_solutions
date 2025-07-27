#![allow(dead_code)]
pub fn convert_to_base7(num: i32) -> String {
    let mut num = num;
    let (mut res, mut base) = (0, 1);
    while num != 0 {
        res += base * (num % 7);
        base *= 10;
        num /= 7;
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_base7_basic() {
        assert_eq!(convert_to_base7(100), "202".to_string());
        assert_eq!(convert_to_base7(-7), "-10".to_string());
    }
}
