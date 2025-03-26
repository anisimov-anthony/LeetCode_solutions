#[allow(dead_code)]
pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    let (x1, y1) = (points[0][0], points[0][1]);
    let (x2, y2) = (points[1][0], points[1][1]);
    let (x3, y3) = (points[2][0], points[2][1]);

    if (x1 == x2 && y1 == y2) || (x2 == x3 && y2 == y3) || (x1 == x3 && y1 == y3) {
        return false;
    }

    let area = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2);
    area != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_boomerang_basic() {
        assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
        assert_eq!(
            is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            false
        );
    }
}
