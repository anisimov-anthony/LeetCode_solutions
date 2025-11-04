#![allow(dead_code)]
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    (1..10)
        .map(|length| {
            (0..9 - length)
                .map(|i| ("123456789"[i..=i + length]).parse::<i32>().unwrap())
                .filter(|&num| low <= num && num <= high)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_digits_basic() {
        assert_eq!(sequential_digits(100, 300), vec![123, 234]);
        assert_eq!(
            sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
}
