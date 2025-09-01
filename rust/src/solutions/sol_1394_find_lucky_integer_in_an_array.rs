#![allow(dead_code)]
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut freqs = std::collections::HashMap::new();
    for el in arr.iter() {
        freqs.entry(el).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut candidates = Vec::new();
    for (el, fr) in freqs.iter() {
        if *el == fr {
            candidates.push(el);
        }
    }
    if let Some(max) = candidates.iter().max() {
        return ***max;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lucky_basic() {
        assert_eq!(find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }
}
