#![allow(dead_code)]
pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut freqs = std::collections::HashMap::new();
    for item in items1.iter() {
        freqs
            .entry(item[0])
            .and_modify(|fr| *fr += item[1])
            .or_insert(item[1]);
    }
    for item in items2.iter() {
        freqs
            .entry(item[0])
            .and_modify(|fr| *fr += item[1])
            .or_insert(item[1]);
    }

    for (&nm, &fr) in freqs.iter() {
        result.push(vec![nm, fr]);
    }

    result.sort_by(|a, b| a[0].cmp(&b[0]));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_similar_items_basic() {
        assert_eq!(
            merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            ),
            vec![[1, 6], [3, 9], [4, 5]]
        );
        assert_eq!(
            merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
        assert_eq!(
            merge_similar_items(
                vec![vec![1, 3], vec![2, 2]],
                vec![vec![7, 1], vec![2, 2], vec![1, 4]]
            ),
            vec![vec![1, 7], vec![2, 4], vec![7, 1]]
        );
    }
}
