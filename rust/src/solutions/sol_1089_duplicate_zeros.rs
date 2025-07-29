#![allow(dead_code)]
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    // corner cases
    if arr.is_empty() || arr.len() == 0 {
        return;
    }

    // main case
    let mut v = Vec::new();
    for (i, &item) in arr.iter().enumerate() {
        if item == 0 {
            v.push(i);
        }
    }
    v.reverse();
    for &i in v.iter() {
        arr.insert(i, 0);
        arr.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_zeros_basic() {
        let mut input = vec![1, 0, 2, 3, 0, 4, 5, 0];
        let valid_output = vec![1, 0, 0, 2, 3, 0, 0, 4];
        duplicate_zeros(&mut input);
        assert_eq!(input, valid_output);

        let mut input = vec![1, 2, 3];
        let valid_output = vec![1, 2, 3];
        duplicate_zeros(&mut input);
        assert_eq!(input, valid_output);
    }
}
