#![allow(dead_code)]
pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = box_grid.len();
    let n = box_grid[0].len();
    let mut res = vec![vec!['.'; m]; n];

    for (idx, row) in box_grid.iter().enumerate() {
        let mut empty_row = n;
        for (col_idx, &cell) in row.iter().enumerate().rev() {
            match &cell {
                '#' => {
                    empty_row -= 1;
                    res[empty_row][m - idx - 1] = cell;
                }
                '*' => {
                    res[col_idx][m - idx - 1] = '*';
                    empty_row = col_idx;
                }
                _ => {}
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_the_box_basic() {
        assert_eq!(
            rotate_the_box(vec![vec!['#', '.', '#']]),
            vec![vec!['.'], vec!['#'], vec!['#']]
        );
        assert_eq!(
            rotate_the_box(vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.']
            ]),
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        )
    }
}
