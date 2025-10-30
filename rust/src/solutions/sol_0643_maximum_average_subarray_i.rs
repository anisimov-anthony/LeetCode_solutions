#![allow(dead_code)]
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    nums.windows(k as usize)
        .map(|wd| wd.iter().sum::<i32>())
        .max()
        .unwrap() as f64
        / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average_basic() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75f64);
        assert_eq!(find_max_average(vec![5], 1), 5f64);
    }
}
