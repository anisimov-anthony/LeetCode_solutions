#![allow(dead_code)]
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut points = points;
    points.sort_by(|a, b| (a[0].pow(2) + a[1].pow(2)).cmp(&(b[0].pow(2) + b[1].pow(2))));
    points[0..(k as usize)].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest_basic() {
        assert_eq!(
            k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }
}
