#![allow(dead_code)]
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let length = nums.len() as i32;
    match k {
        1 => return *nums.iter().max().unwrap(),
        k if k == length => return *nums.iter().min().unwrap(),
        _ => {
            let mut nums = nums;
            nums.sort_by(|a, b| b.cmp(&a));
            return nums[(k - 1) as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest_basic() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
