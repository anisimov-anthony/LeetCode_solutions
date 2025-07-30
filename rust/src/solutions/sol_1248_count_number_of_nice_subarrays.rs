#![allow(dead_code)]
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut count = vec![0; n + 1];
    let k = k as usize;
    count[0] = 1;
    let mut prev = 0;
    let mut res = 0;
    for i in 0..n {
        if nums[i] % 2 == 1 {
            prev += 1;
        }
        count[prev] += 1;
        if prev >= k {
            res += count[prev - k];
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_number_of_subarrays_basic() {
        assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(
            number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
