#![allow(dead_code)]
pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    fn is_arithmetic(mut arr: Vec<i32>) -> bool {
        if arr.len() < 2 {
            return true;
        }
        arr.sort_unstable();
        let diff = arr[1] - arr[0];
        arr.windows(2).all(|wd| wd[1] - wd[0] == diff)
    }

    l.iter()
        .zip(r)
        .map(|(li, ri)| is_arithmetic(nums[(*li as usize)..=(ri as usize)].to_vec()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_arithmetic_subarrays_basic() {
        assert_eq!(
            check_arithmetic_subarrays(vec![4, 6, 5, 9, 3, 7], vec![0, 0, 2], vec![2, 3, 5]),
            vec![true, false, true]
        );
        assert_eq!(
            check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
