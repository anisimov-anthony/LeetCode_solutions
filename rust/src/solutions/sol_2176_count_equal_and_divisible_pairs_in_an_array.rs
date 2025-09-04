#![allow(dead_code)]
pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut pairs: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        pairs.entry(*num).or_insert_with(Vec::new).push(idx);
    }

    let mut res = 0;
    for (_, idxs) in pairs.iter() {
        for i in 0..idxs.len() {
            for j in (i + 1)..idxs.len() {
                if (idxs[i as usize] * idxs[j as usize]) as i32 % k == 0 {
                    res += 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pairs_basic() {
        assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
