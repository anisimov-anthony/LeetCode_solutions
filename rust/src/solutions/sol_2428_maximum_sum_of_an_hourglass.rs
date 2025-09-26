#![allow(dead_code)]
pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut h_sums = Vec::new();
    for i in 0..=(grid.len() - 3) {
        for j in 0..=(grid[i].len() - 3) {
            let first_l: i32 = grid[i][j..(j + 3)].iter().sum();
            let second_l = grid[i + 1][j + 1];
            let third_l: i32 = grid[i + 2][j..(j + 3)].iter().sum();
            h_sums.push(first_l + second_l + third_l);
        }
    }
    *h_sums.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_basic() {
        assert_eq!(
            max_sum(vec![
                vec![6, 2, 1, 3],
                vec![4, 2, 1, 5],
                vec![9, 2, 8, 7],
                vec![4, 1, 2, 9]
            ]),
            30
        );
        assert_eq!(
            max_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            35
        );
    }
}
