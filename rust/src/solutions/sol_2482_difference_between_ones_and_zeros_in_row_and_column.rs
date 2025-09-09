#![allow(dead_code)]
pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ones_row = std::collections::HashMap::new();
    let mut ones_column = std::collections::HashMap::new();
    let mut zeros_row = std::collections::HashMap::new();
    let mut zeros_column = std::collections::HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                ones_row.entry(i).and_modify(|num| *num += 1).or_insert(1);

                ones_column
                    .entry(j)
                    .and_modify(|num| *num += 1)
                    .or_insert(1);
            } else {
                zeros_row.entry(i).and_modify(|num| *num += 1).or_insert(1);

                zeros_column
                    .entry(j)
                    .and_modify(|num| *num += 1)
                    .or_insert(1);
            }
        }
    }

    let mut res = Vec::with_capacity(grid.len());
    for i in 0..grid.len() {
        let mut rw = Vec::with_capacity(grid[0].len());
        for j in 0..grid[0].len() {
            rw.push(
                (*ones_row.get(&i).unwrap_or(&0) as i32 + *ones_column.get(&j).unwrap_or(&0) as i32
                    - *zeros_row.get(&i).unwrap_or(&0) as i32
                    - *zeros_column.get(&j).unwrap_or(&0) as i32) as i32,
            );
        }
        res.push(rw);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones_minus_zeros_basic() {
        assert_eq!(
            ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
        assert_eq!(
            ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            vec![vec![5, 5, 5], vec![5, 5, 5]]
        );
    }
}
