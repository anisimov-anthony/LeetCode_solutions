#[allow(dead_code)]
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut freqs = std::collections::HashMap::new();
    for nm in nums.iter() {
        freqs.entry(nm).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let mut freqs: Vec<_> = freqs.into_iter().collect();
    freqs.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));

    let mut result = Vec::new();
    for (num, fr) in freqs.iter() {
        for _ in 0..*fr {
            result.push(**num);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort_basic() {
        assert_eq!(
            frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
        assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
        assert_eq!(
            frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
