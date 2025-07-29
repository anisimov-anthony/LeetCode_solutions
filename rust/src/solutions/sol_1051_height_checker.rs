#![allow(dead_code)]
pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut needed = heights.clone();
    needed.sort();

    let mut result = 0;
    for i in 0..heights.len() {
        if heights[i] != needed[i] {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_checker_basic() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
        assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
