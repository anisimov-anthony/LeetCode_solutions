#![allow(dead_code)]
pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mins_in_rows: std::collections::HashSet<_> = matrix
        .iter()
        .map(|row| *row.iter().min().unwrap())
        .collect();

    let maxs_in_columns: std::collections::HashSet<_> = (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).max().unwrap())
        .collect();

    mins_in_rows
        .intersection(&maxs_in_columns)
        .map(|&n| n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucky_numbers_basic() {
        assert_eq!(
            lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
        assert_eq!(
            lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
        assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
    }
}
