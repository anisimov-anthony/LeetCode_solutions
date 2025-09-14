#![allow(dead_code)]
pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let idxs: Vec<usize> = nums
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == key)
        .map(|fl| fl.0)
        .collect();

    let mut res = Vec::new();
    for nm_idx in 0..nums.len() {
        for idx in idxs.iter() {
            if idx.abs_diff(nm_idx) <= k as usize {
                res.push(nm_idx as i32);
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_k_distant_indices_basic() {
        assert_eq!(
            find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
    }
}
