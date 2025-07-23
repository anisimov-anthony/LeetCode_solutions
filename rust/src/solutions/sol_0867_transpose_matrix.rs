#![allow(dead_code)]
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return matrix;
    }

    let mut result = Vec::with_capacity(matrix[0].len());
    for j in 0..matrix[0].len() {
        let mut row = Vec::new();
        for i in 0..matrix.len() {
            row.push(matrix[i][j]);
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_basic() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );

        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}
