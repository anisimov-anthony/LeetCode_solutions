#![allow(dead_code)]
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut q = std::collections::VecDeque::with_capacity(m * n);
    let mut n_islands = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                n_islands += 1;
                q.push_back((i, j));
                while !q.is_empty() {
                    for _ in 0..q.len() {
                        let (k, l) = q.pop_front().unwrap();
                        if k < m && l < n && grid[k][l] == '1' {
                            grid[k][l] = '0';
                            for w in [0, 1, 0, !0, 0].windows(2) {
                                q.push_back((k.wrapping_add(w[0]), l.wrapping_add(w[1])));
                            }
                        }
                    }
                }
            }
        }
    }
    n_islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_islands_basic() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
