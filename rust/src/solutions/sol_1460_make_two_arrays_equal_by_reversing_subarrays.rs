#![allow(dead_code)]
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut arr = arr;
    let mut target = target;
    arr.sort();
    target.sort();

    arr == target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_equal_basic() {
        assert!(can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
        assert!(can_be_equal(vec![7], vec![7]));
        assert!(!can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
