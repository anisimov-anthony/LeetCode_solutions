#![allow(dead_code)]
pub fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut sums = std::collections::HashSet::new();
    for pr in nums.windows(2) {
        if sums.contains(&(pr[0] + pr[1])) {
            return true;
        } else {
            sums.insert(pr[0] + pr[1]);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subarrays_basic() {
        assert!(find_subarrays(vec![4, 2, 4]));
        assert!(!find_subarrays(vec![1, 2, 3, 4, 5]));
        assert!(find_subarrays(vec![0, 0, 0]));
    }
}
