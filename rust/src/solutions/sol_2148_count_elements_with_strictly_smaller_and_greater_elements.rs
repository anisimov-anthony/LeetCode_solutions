#![allow(dead_code)]
pub fn count_elements(nums: Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    if max == min {
        return 0;
    }

    let min_count = nums.iter().filter(|&nm| nm == min).count() as i32;
    let max_count = nums.iter().filter(|&nm| nm == max).count() as i32;

    nums.len() as i32 - max_count - min_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_elements_basic() {
        assert_eq!(count_elements(vec![11, 7, 2, 15]), 2);
        assert_eq!(count_elements(vec![-3, 3, 3, 90]), 2);
    }
}
