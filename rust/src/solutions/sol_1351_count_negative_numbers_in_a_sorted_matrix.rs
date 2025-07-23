#![allow(dead_code)]
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    if grid.is_empty() {
        return result;
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] < 0 {
                result += (grid[i].len() - j) as i32;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negatives_basic() {
        assert_eq!(
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
