#![allow(dead_code)]
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let v: Vec<i32> = (1..=nums.len()).map(|x| x as i32).collect();
    let mut disap: std::collections::HashSet<i32> = v.into_iter().collect();

    for num in nums.iter() {
        disap.remove(num);
    }
    let mut result: Vec<i32> = disap.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers_basic() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
