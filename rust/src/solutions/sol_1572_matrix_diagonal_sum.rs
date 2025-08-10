#![allow(dead_code)]
pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    if mat.len() == 1 {
        return mat[0][0];
    }

    let mut result: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = mat.len() as i32 - 1;
    let mut direction: i32 = 1;
    for row in mat.iter() {
        if (left < right) || (left > right) {
            result += row[left as usize];
            result += row[right as usize];
        } else {
            result += row[left as usize];
            direction = -1;
        }
        left += direction;
        right -= direction;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_sum_basic() {
        assert_eq!(
            diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );

        assert_eq!(
            diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8
        );

        assert_eq!(diagonal_sum(vec![vec![5]]), 5);
    }
}
