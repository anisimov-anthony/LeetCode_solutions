#[allow(dead_code)]
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i() {
        assert_eq!(hamming_distance(1, 4), 2);
        assert_eq!(hamming_distance(3, 1), 1);
    }
}
