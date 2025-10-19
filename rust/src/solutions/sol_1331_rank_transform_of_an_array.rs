#![allow(dead_code)]
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut sorted = arr.clone();
    sorted.sort_unstable();

    let mut rank = 1;
    let mut prev = sorted[0];
    let ranks: std::collections::HashMap<_, _> = sorted
        .iter()
        .map(|&num| {
            if prev != num {
                rank += 1;
                prev = num;
            }

            (num, rank)
        })
        .collect();

    arr.iter().map(|num| ranks[num]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_rank_transform_basic() {
        assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
        assert_eq!(array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
        assert_eq!(
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
