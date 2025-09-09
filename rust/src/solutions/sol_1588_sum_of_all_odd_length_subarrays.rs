#![allow(dead_code)]
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut offset = 1;
    while offset <= arr.len() {
        res += arr
            .windows(offset)
            .fold(0, |acc, x| acc + x.iter().sum::<i32>());
        offset += 2;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_odd_length_subarrays_basic() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
