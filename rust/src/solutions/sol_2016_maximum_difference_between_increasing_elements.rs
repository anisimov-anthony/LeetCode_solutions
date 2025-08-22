#![allow(dead_code)]
pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return -1;
    }

    let mut res = -1;
    for i in 1..nums.len() {
        let lmin = nums[..i].iter().min().unwrap();
        let rmax = nums[i..].iter().max().unwrap();
        if lmin != rmax {
            res = res.max(rmax - lmin);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_difference_basic() {
        assert_eq!(maximum_difference(vec![7, 1, 5, 4]), 4);
        assert_eq!(maximum_difference(vec![9, 4, 3, 2]), -1);
        assert_eq!(maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
