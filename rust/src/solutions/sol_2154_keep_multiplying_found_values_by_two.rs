#![allow(dead_code)]
pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut target = original;
    while nums.contains(&target) {
        target = target * 2;
    }
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_final_value_basic() {
        assert_eq!(find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
        assert_eq!(find_final_value(vec![2, 7, 9], 4), 4);
    }
}
