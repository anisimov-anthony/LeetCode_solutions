#![allow(dead_code)]
pub fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut cost = cost;
    cost.sort_unstable();
    for i in 0..cost.len() {
        if i % 3 < 2 {
            res += cost[cost.len() - i - 1];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost_basic() {
        assert_eq!(minimum_cost(vec![1, 2, 3]), 5);
        assert_eq!(minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
        assert_eq!(minimum_cost(vec![5, 5]), 10);
    }
}
