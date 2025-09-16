#![allow(dead_code)]
pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
    order
        .into_iter()
        .filter(|ord| friends.contains(&ord))
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_order_basic() {
        assert_eq!(
            recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]),
            vec![3, 1, 4]
        );
        assert_eq!(recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]), vec![5, 2]);
    }
}
