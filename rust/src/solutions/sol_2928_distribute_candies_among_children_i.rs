#![allow(dead_code)]
pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    let mut counter = 0;

    for i in 0..=limit {
        for j in 0..=limit {
            for k in 0..=limit {
                if i + j + k > n {
                    break;
                }
                if i + j + k == n {
                    counter += 1;
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies_basic() {
        assert_eq!(distribute_candies(5, 2), 3);
        assert_eq!(distribute_candies(3, 3), 10);
    }
}
