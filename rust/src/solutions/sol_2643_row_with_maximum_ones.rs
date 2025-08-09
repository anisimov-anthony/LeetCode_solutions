#![allow(dead_code)]
pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0, 0];
    for (idx, row) in mat.iter().enumerate() {
        let current_ones: i32 = row.iter().sum();
        if current_ones > result[1] {
            result[0] = idx as i32;
            result[1] = current_ones as i32;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_and_maximum_ones_basic() {
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            vec![0, 1]
        );
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
