#![allow(dead_code)]
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut freqs = std::collections::HashMap::new();
    for i in arr.iter() {
        freqs.entry(i).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut freqs = freqs.into_iter().map(|(_i, fr)| fr).collect::<Vec<i32>>();
    freqs.sort();
    if freqs.len() == 0 || freqs.len() == 1 {
        return true;
    }

    for i in 1..freqs.len() {
        if freqs[i] == freqs[i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_occurrences_basic() {
        assert!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert!(!unique_occurrences(vec![1, 2]));
        assert!(unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]));
    }
}
