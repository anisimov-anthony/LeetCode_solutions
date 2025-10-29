#![allow(dead_code)]
pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        result += (nums[left].to_string() + (&nums[right].to_string()))
            .parse::<i64>()
            .unwrap();

        left += 1;
        right -= 1;
    }
    if left == right {
        result += nums[left] as i64;
    }
    result as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_array_conc_val_basic() {
        assert_eq!(find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(find_the_array_conc_val(vec![5, 14, 13, 8, 12]), 673);
    }
}
