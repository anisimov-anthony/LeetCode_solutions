#![allow(dead_code)]
pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut freqs = std::collections::HashMap::new();
    for num in nums.iter() {
        freqs.entry(num).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let freqs: Vec<_> = freqs.into_iter().map(|(_, fr)| fr).collect();
    let maximum = freqs.iter().max();

    let mut result = 0;
    for fr in freqs.iter() {
        if fr == maximum.unwrap() {
            result += fr;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency_elements_basic() {
        assert_eq!(max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
