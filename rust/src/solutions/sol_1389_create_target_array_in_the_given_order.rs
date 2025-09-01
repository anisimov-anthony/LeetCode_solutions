#![allow(dead_code)]
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums.len());
    nums.iter()
        .zip(index.iter())
        .for_each(|(nm, idx)| res.insert(*idx as usize, *nm));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_target_array_basic() {
        assert_eq!(
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(create_target_array(vec![1], vec![0]), vec![1]);
    }
}
