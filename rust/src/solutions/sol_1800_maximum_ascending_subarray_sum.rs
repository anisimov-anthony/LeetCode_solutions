#![allow(dead_code)]
pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    fn is_ascending(arr: &Vec<i32>) -> bool {
        arr.windows(2).all(|ar| ar[0] < ar[1])
    }
    let mut res = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if is_ascending(&nums[i..=j].to_vec()) {
                res = res.max(nums[i..=j].iter().sum::<i32>());
            } else {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ascending_sum_basic() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
    }
}
