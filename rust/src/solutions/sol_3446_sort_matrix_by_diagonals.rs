#![allow(dead_code)]
pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if grid.len() <= 1 {
        return grid;
    }
    let mut grid = grid;

    // bottom sorting
    for i in 0..grid.len() {
        let mut diag = Vec::new();
        for j in 0..grid.len() - i {
            diag.push(grid[i + j][j]);
        }
        diag.sort_unstable_by(|a, b| b.cmp(&a));

        for j in 0..grid.len() - i {
            grid[i + j][j] = diag[j];
        }
    }

    // top sorting
    for i in 1..grid.len() {
        let mut diag = Vec::new();
        for j in 0..grid.len() - i {
            diag.push(grid[j][i + j]);
        }
        diag.sort_unstable_by(|a, b| a.cmp(&b));

        for j in 0..grid.len() - i {
            grid[j][i + j] = diag[j];
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_matrix_basic() {
        assert_eq!(
            sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]]),
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]
        );
        assert_eq!(
            sort_matrix(vec![vec![0, 1], vec![1, 2]]),
            vec![vec![2, 1], vec![1, 0]]
        );
        assert_eq!(sort_matrix(vec![vec![1]]), vec![vec![1]]);
    }
}
