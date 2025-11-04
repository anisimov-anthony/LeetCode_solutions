#![allow(dead_code)]
pub fn min_operations(s: String) -> i32 {
    let digits: Vec<_> = s.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    let mut first_case = 0;
    let mut second_case = 0;
    for (i, &dg) in digits.iter().enumerate() {
        if i % 2 == 0 {
            if dg != 0 {
                first_case += 1;
            } else {
                second_case += 1;
            }
        } else {
            if dg != 0 {
                second_case += 1;
            } else {
                first_case += 1;
            }
        }
    }

    first_case.min(second_case)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_basic() {
        assert_eq!(min_operations("0100".to_string()), 1);
        assert_eq!(min_operations("10".to_string()), 0);
        assert_eq!(min_operations("1111".to_string()), 2);
    }
}
