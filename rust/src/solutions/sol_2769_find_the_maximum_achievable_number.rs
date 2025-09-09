#![allow(dead_code)]
pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + 2 * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_maximum_achievable_x_basic() {
        assert_eq!(the_maximum_achievable_x(4, 1), 6);
        assert_eq!(the_maximum_achievable_x(3, 2), 7);
    }
}
