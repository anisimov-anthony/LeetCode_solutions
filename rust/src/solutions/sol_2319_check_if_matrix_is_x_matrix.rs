#![allow(dead_code)]
pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    let mut left: i32 = 0;
    let mut right: i32 = grid.len() as i32 - 1;
    let mut meeted = false;
    let mut level = 0;
    let mut direction: i32 = 1;
    while (left != -1) || direction != -1 {
        if (grid[level][left as usize] != 0)
            && (grid[level][right as usize] != 0)
            && grid[level]
                .iter()
                .enumerate()
                .filter(|en| en.0 != left as usize && en.0 != right as usize)
                .all(|fl| *fl.1 == 0)
        {
            if left == right - 1 {
                if !meeted {
                    meeted = true;
                } else {
                    direction = -1;
                    left += direction;
                    right -= direction;
                }
            } else {
                if left == right {
                    direction = -1;
                }
                left += direction;
                right -= direction;
            }
            level += 1;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_x_matrix_basic() {
        assert!(check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2]
        ]));

        assert!(!check_x_matrix(vec![
            vec![5, 7, 0],
            vec![0, 3, 1],
            vec![0, 5, 0]
        ]));
    }
}
