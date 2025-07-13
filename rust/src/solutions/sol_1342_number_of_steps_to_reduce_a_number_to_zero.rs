#[allow(dead_code)]
pub fn number_of_steps(num: i32) -> i32 {
    let mut result = 0;
    let mut num = num;
    while num != 0 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num -= 1;
        }
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_steps_basic() {
        assert_eq!(number_of_steps(14), 6);
        assert_eq!(number_of_steps(123), 12);
    }
}
