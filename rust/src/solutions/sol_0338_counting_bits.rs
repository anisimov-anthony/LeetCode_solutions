#![allow(dead_code)]
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|i| i.count_ones() as i32).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits_basic() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
