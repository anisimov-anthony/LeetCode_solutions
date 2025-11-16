#![allow(dead_code)]
pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut freqs = std::collections::HashMap::new();
    for nm in nums.iter() {
        freqs.entry(nm).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let twices: Vec<i32> = freqs
        .iter()
        .filter(|&(_, freq)| *freq == 2)
        .map(|fl| **fl.0)
        .collect();

    if twices.len() == 0 {
        return 0;
    }

    let mut result = twices[0];
    for i in 1..twices.len() {
        result = result ^ twices[i];
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_numbers_xor_basic() {
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 3]), 0);
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
