#[allow(dead_code)]
pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    return (low..=high)
        .filter(|x| {
            let number = x.to_string();
            let digits = number.as_bytes();
            if number.len() % 2 == 1 {
                return false;
            }

            let left_sum: i32 = digits[0..number.len() / 2]
                .iter()
                .map(|x| (x - b'0') as i32)
                .sum();

            let right_sum: i32 = digits[number.len() / 2..]
                .iter()
                .map(|x| (x - b'0') as i32)
                .sum();

            left_sum == right_sum
        })
        .count() as i32;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_symmetric_integers_basic() {
        assert_eq!(count_symmetric_integers(1, 100), 9);
        assert_eq!(count_symmetric_integers(1200, 1230), 4);
    }
}
