#![allow(dead_code)]
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left = Vec::new();
    let mut middle = Vec::new();
    let mut right = Vec::new();
    for num in nums.iter() {
        match num.cmp(&pivot) {
            std::cmp::Ordering::Less => left.push(*num),
            std::cmp::Ordering::Equal => middle.push(*num),
            std::cmp::Ordering::Greater => right.push(*num),
        }
    }
    left.extend(&middle);
    left.extend(&right);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_array_basic() {
        assert_eq!(
            pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
        assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
    }
}
