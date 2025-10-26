#![allow(dead_code)]
pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut freqs = std::collections::HashMap::new();
    for nm in nums.iter() {
        freqs.entry(nm).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut deleted_cnt = 0;
    let mut leftover_counter = 0;
    for (_, freq) in freqs.iter() {
        deleted_cnt += freq / 2;
        leftover_counter += freq % 2;
    }

    vec![deleted_cnt as i32, leftover_counter as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_pairs_basic() {
        assert_eq!(number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]), vec![3, 1]);
        assert_eq!(number_of_pairs(vec![1, 1]), vec![1, 0]);
        assert_eq!(number_of_pairs(vec![0]), vec![0, 1]);
    }
}
