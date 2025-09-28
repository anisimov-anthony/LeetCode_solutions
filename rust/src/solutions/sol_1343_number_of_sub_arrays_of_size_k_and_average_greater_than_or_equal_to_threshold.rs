#![allow(dead_code)]
pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    arr.windows(k as usize)
        .filter(|wd| wd.iter().sum::<i32>() / k >= threshold)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_subarrays_basic() {
        assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
        assert_eq!(
            num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
    }
}
