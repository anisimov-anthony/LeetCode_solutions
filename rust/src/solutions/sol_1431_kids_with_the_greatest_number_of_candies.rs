#[allow(dead_code)]
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max: i32 = *candies.iter().max().unwrap();
    let mut result = Vec::with_capacity(candies.len());
    for i in 0..candies.len() {
        if candies[i] + extra_candies >= max {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_with_candies_basic() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
