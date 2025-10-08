#![allow(dead_code)]
pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut first = std::collections::HashSet::new();
    let mut second = std::collections::HashSet::new();

    for nm in nums.iter() {
        if !first.contains(&nm) {
            first.insert(nm);
        } else {
            if second.contains(&nm) {
                return false;
            } else {
                second.insert(nm);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible_to_split_basic() {
        assert!(is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
        assert!(!is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
