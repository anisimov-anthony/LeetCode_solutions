#![allow(dead_code)]
pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_bit_flips_basic() {
        assert_eq!(min_bit_flips(10, 7), 3);
        assert_eq!(min_bit_flips(3, 4), 3);
    }
}
