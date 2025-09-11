#![allow(dead_code)]
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .map(|(i, &val)| {
            nums.iter()
                .enumerate()
                .filter(|(ni, nval)| *ni != i && **nval < val)
                .count() as i32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current_basic() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
