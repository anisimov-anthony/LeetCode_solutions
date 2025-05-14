#[allow(dead_code)]
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut duplicates: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for i in 0..nums.len() {
        duplicates.entry(nums[i]).or_insert(vec![]).push(i as i32);
    }

    for (_, duplic_idxs) in duplicates {
        for i in 0..duplic_idxs.len() - 1 {
            let j = i + 1;
            if (duplic_idxs[j] - duplic_idxs[i]) <= k {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate_basic() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
