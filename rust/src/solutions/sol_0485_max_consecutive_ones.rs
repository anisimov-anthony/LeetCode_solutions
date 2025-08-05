#![allow(dead_code)]
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    nums.iter().fold(0, |acc, a| {
        if *a == 1 {
            cnt += 1
        } else {
            cnt = 0
        }
        acc.max(cnt)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones_basic() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
