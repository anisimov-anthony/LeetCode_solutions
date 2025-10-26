#![allow(dead_code)]
pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();
    let mut left = 0;
    let mut right = people.len() as i32 - 1;

    let mut result = 0;
    while left <= right {
        if left < right && people[left as usize] <= limit - people[right as usize] {
            left += 1;
        }
        right -= 1;
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rescue_boats_basic() {
        assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
        assert_eq!(num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
        assert_eq!(num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
