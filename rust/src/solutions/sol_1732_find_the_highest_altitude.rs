#![allow(dead_code)]
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    gain.iter()
        .scan(0, |prev_sum, elem| {
            *prev_sum = *prev_sum + elem;
            Some(*prev_sum)
        })
        .into_iter()
        .max()
        .unwrap()
        .max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_altitude_basic() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
