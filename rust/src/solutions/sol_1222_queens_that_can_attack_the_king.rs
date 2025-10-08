#![allow(dead_code)]
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    // + directions
    // -
    for i in (0..king[0] as usize).rev() {
        if queens.contains(&vec![i as i32, king[1]]) {
            res.push(vec![i as i32, king[1]]);
            break;
        }
    }
    for i in (king[0] as usize + 1)..8 {
        if queens.contains(&vec![i as i32, king[1]]) {
            res.push(vec![i as i32, king[1]]);
            break;
        }
    }
    // |
    for i in (0..king[1] as usize).rev() {
        if queens.contains(&vec![king[0], i as i32]) {
            res.push(vec![king[0], i as i32]);
            break;
        }
    }
    for i in (king[1] as usize + 1)..8 {
        if queens.contains(&vec![king[0], i as i32]) {
            res.push(vec![king[0], i as i32]);
            break;
        }
    }

    // x directions
    // /
    let (mut x, mut y) = (king[0], king[1]);
    while x <= 8 && y >= 0 {
        if queens.contains(&vec![x as i32, y as i32]) {
            res.push(vec![x as i32, y as i32]);
            break;
        }
        x += 1;
        y -= 1;
    }
    let (mut x, mut y) = (king[0], king[1]);
    while x >= 0 && y <= 8 {
        if queens.contains(&vec![x as i32, y as i32]) {
            res.push(vec![x as i32, y as i32]);
            break;
        }
        x -= 1;
        y += 1;
    }

    // \
    let (mut x, mut y) = (king[0], king[1]);
    while x >= 0 && y >= 0 {
        if queens.contains(&vec![x as i32, y as i32]) {
            res.push(vec![x as i32, y as i32]);
            break;
        }
        x -= 1;
        y -= 1;
    }
    let (mut x, mut y) = (king[0], king[1]);
    while x <= 8 && y <= 8 {
        if queens.contains(&vec![x as i32, y as i32]) {
            res.push(vec![x as i32, y as i32]);
            break;
        }
        x += 1;
        y += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queens_attackthe_king_basic() {
        assert_eq!(
            queens_attackthe_king(
                vec![
                    vec![0, 1],
                    vec![1, 0],
                    vec![4, 0],
                    vec![0, 4],
                    vec![3, 3],
                    vec![2, 4]
                ],
                vec![0, 0]
            ),
            vec![vec![1, 0], vec![0, 1], vec![3, 3]]
        );
        assert_eq!(
            queens_attackthe_king(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 4],
                    vec![4, 5]
                ],
                vec![3, 3]
            ),
            vec![vec![3, 4], vec![2, 2], vec![4, 4]]
        );
    }
}
