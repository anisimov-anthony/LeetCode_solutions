#![allow(dead_code)]
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(arr.len());
    for i in 0..arr.len() {
        result.push(*arr[i + 1..].iter().max().unwrap_or(&-1));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_elements_basic() {
        assert_eq!(
            replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
        assert_eq!(replace_elements(vec![400]), vec![-1]);
    }
}
