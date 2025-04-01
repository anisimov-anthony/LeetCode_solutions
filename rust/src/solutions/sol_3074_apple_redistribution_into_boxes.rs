#[allow(dead_code)]
pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let mut sum_of_apples: i32 = apple.iter().sum();
    let mut new_capacity: Vec<i32> = capacity.clone();
    new_capacity.sort_unstable();
    while sum_of_apples > 0 && new_capacity.len() > 0 {
        sum_of_apples -= new_capacity[new_capacity.len() - 1];
        new_capacity.pop();
    }

    (new_capacity.capacity() - new_capacity.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_boxes_basic() {
        assert_eq!(minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]), 2);
        assert_eq!(minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
}
