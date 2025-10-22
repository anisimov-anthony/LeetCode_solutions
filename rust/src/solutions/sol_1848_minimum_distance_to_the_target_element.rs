#![allow(dead_code)]
pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    let mut res = i32::MAX;
    for (i, &val) in nums.iter().enumerate() {
        if val == target {
            res = res.min(i.abs_diff(start as usize) as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_distance_basic() {
        assert_eq!(get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
        assert_eq!(get_min_distance(vec![1], 1, 0), 0);
        assert_eq!(
            get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
            0
        );
    }
}
