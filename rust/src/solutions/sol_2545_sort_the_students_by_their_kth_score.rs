#![allow(dead_code)]
pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut score = score;
    score.sort_by(|a, b| (b[k as usize]).cmp(&a[k as usize]));
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_the_students_basic() {
        assert_eq!(
            sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            ),
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]]
        );

        assert_eq!(
            sort_the_students(vec![vec![3, 4], vec![5, 6]], 0),
            vec![vec![5, 6], vec![3, 4]]
        );
    }
}
