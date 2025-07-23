#![allow(dead_code)]
pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for i in left..(right + 1) {
        let digits: Vec<_> = i
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();

        let mut need_to_add = true;
        for dg in digits.iter() {
            if (*dg == 0) || (i as u32 % *dg != 0) {
                need_to_add = false;
            }
        }

        if need_to_add {
            result.push(i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_dividing_numbers_basic() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }
}
