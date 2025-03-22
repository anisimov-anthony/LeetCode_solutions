#[allow(dead_code)]
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len() * 2);
    for i in 0..nums.len() * 2 {
        result.push(nums[i % nums.len()]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_concatenation_basic() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
