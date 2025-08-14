#![allow(dead_code)]
pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut sequences: Vec<(usize, usize)> = vec![];

    for i in 0..nums.len() {
        for j in (i + 1..nums.len()).rev() {
            if nums[j] - nums[i] == 1 {
                sequences.push((i, j));
            }
        }
    }

    sequences
        .into_iter()
        .map(|(i, j)| (j - i + 1) as i32)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lhs_basic() {
        assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
        assert_eq!(find_lhs(vec![1, 2, 3, 4]), 2);
        assert_eq!(find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
