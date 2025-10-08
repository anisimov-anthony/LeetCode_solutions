#![allow(dead_code)]
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut covered = std::collections::HashMap::new();
    for rn in ranges.iter() {
        for i in rn[0]..=rn[1] {
            covered.entry(i).and_modify(|fr| *fr += 1).or_insert(1);
        }
    }

    for i in left..=right {
        if !covered.contains_key(&i) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_covered_basic() {
        assert!(is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5));
        assert!(!is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21));
    }
}
