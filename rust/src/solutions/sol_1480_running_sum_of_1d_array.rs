#[allow(dead_code)]
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum_basic() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
