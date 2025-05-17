#[allow(dead_code)]
// bad solution
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut freqs = std::collections::HashMap::new();

    for i in nums.iter() {
        if freqs.contains_key(i) {
            let fr = freqs.get_mut(i).unwrap();
            *fr += 1;
        } else {
            freqs.insert(i, 1);
        }
    }

    for (key, value) in freqs.iter() {
        if *value > (nums.len() / 3) {
            result.push(**key);
        }
    }

    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element_basic() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(majority_element(vec![1]), vec![1]);
        assert_eq!(majority_element(vec![1, 2]), vec![1, 2])
    }
}
