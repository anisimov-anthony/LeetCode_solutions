#![allow(dead_code)]
pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(&a));
    let mut candidates: Vec<i32> = Vec::new();
    let full_sum: i32 = nums.iter().sum();

    for &nm in nums.iter() {
        candidates.push(nm);
        let local_sum: i32 = candidates.iter().sum();
        if local_sum > full_sum - local_sum {
            break;
        }
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_subsequence_basic() {
        assert_eq!(min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    }
}
