#![allow(dead_code)]
pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = vec![vec![0i32; cols - k + 1]; rows - k + 1];

    for i in 0..rows - k + 1 {
        for j in 0..cols - k + 1 {
            let mut min_diff = i32::MAX;

            let mut locals = Vec::with_capacity(k * k);
            for r in i..i + k {
                for c in j..j + k {
                    locals.push(grid[r][c]);
                }
            }

            for a in 0..locals.len() {
                for b in a + 1..locals.len() {
                    let diff = locals[a].abs_diff(locals[b]) as i32;
                    if diff != 0 && diff < min_diff {
                        min_diff = diff;
                    }
                }
            }

            result[i][j] = if min_diff == i32::MAX { 0 } else { min_diff };
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_abs_diff_basic() {
        assert_eq!(
            min_abs_diff(vec![vec![1, 8], vec![3, -2]], 2),
            vec![vec![2]]
        );
        assert_eq!(min_abs_diff(vec![vec![3, -1]], 1), vec![vec![0, 0]]);
        assert_eq!(
            min_abs_diff(vec![vec![1, -2, 3], vec![2, 3, 5]], 2),
            vec![vec![1, 2]]
        );
    }
}
