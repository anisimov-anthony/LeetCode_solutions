#![allow(dead_code)]
pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let n = candy_type.len() / 2;
    let mut candies = std::collections::HashSet::new();
    for cnd in candy_type.iter() {
        candies.insert(cnd);
    }

    n.min(candies.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies_basic() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
