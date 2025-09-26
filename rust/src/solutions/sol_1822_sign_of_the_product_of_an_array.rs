#![allow(dead_code)]
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut neg = 0;
    for nm in nums.iter() {
        match nm.cmp(&0) {
            std::cmp::Ordering::Less => neg += 1,
            std::cmp::Ordering::Equal => return 0,
            _ => continue,
        }
    }

    if neg % 2 == 0 {
        return 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_sign_basic() {
        assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
