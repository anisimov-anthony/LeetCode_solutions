#![allow(dead_code)]
pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return true;
    }
    let mut arr = arr;
    arr.sort_unstable();
    let diff = arr[1] - arr[0];
    arr.windows(2).all(|wd| wd[1] - wd[0] == diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_arithmetic_progression_basic() {
        assert!(can_make_arithmetic_progression(vec![3, 5, 1]));
        assert!(!can_make_arithmetic_progression(vec![1, 2, 4]));
    }
}
