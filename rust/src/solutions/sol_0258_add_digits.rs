#[allow(dead_code)]
pub fn add_digits(num: i32) -> i32 {
    if num == 0 {
        0
    } else if num % 9 == 0 {
        9
    } else {
        num % 9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_digits_basic() {
        assert_eq!(add_digits(38), 2);
        assert_eq!(add_digits(0), 0);
    }
}
