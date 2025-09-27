#![allow(dead_code)]
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = std::collections::BinaryHeap::from(stones);
    while stones.len() > 1 {
        let x = stones.pop().unwrap();
        let y = stones.pop().unwrap();
        if x != y {
            stones.push(x - y);
        }
    }
    stones.pop().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_stone_weight_basic() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(last_stone_weight(vec![1]), 1);
    }
}
