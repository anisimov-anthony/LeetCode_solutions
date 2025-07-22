#![allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut bits_freqs = std::collections::HashMap::new();

    fn i32_to_bits(n: i32) -> Vec<u8> {
        (0..32).map(|i| ((n >> i) & 1) as u8).collect()
    }

    for num in nums.iter() {
        for (i, val) in i32_to_bits(*num).iter().enumerate() {
            if *val == 1 {
                bits_freqs.entry(i).and_modify(|fr| *fr += 1).or_insert(1);
            }
        }
    }

    let mut result = 0;
    for (bit, fr) in bits_freqs.iter() {
        if fr % 3 != 0 {
            result |= 1 << bit;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_basic() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
