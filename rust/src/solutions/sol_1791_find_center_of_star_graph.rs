#[allow(dead_code)]
pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    for i in &edges {
        if !i.contains(&edges[0][0]) {
            return edges[0][1];
        }
    }
    return edges[0][0];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_center_basic() {
        assert_eq!(find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
        assert_eq!(
            find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
