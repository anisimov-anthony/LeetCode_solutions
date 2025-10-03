#![allow(dead_code)]
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    points.windows(2).fold(0, |acc, v| {
        acc + (v[0][0] - v[1][0]).abs().max((v[0][1] - v[1][1]).abs())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_to_visit_all_points_basic() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
