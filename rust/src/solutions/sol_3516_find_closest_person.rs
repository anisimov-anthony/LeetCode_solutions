#![allow(dead_code)]
pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    match x.abs_diff(z).cmp(&y.abs_diff(z)) {
        std::cmp::Ordering::Less => return 1,
        std::cmp::Ordering::Equal => return 0,
        std::cmp::Ordering::Greater => return 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_basic() {
        assert_eq!(find_closest(2, 7, 4), 1);
        assert_eq!(find_closest(2, 5, 6), 2);
        assert_eq!(find_closest(1, 5, 3), 0);
    }
}
