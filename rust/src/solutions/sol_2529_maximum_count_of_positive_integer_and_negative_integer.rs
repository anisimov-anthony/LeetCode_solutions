#![allow(dead_code)]
pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut neg = 0;
    let mut zer = 0;
    let mut pos = 0;
    for &nm in nums.iter() {
        if nm < 0 {
            neg += 1;
        }
        if nm == 0 {
            zer += 1;
        } else {
            pos = nums.len() - zer - neg;
        }
    }

    neg.max(pos) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_count_basic() {
        assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
        assert_eq!(maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
        assert_eq!(maximum_count(vec![5, 20, 66, 1314]), 4);
    }
}
