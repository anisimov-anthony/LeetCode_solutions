use std::collections::HashMap;

#[allow(dead_code)]
pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    let mut nums = nums;
    let mut positions: HashMap<i32, Vec<usize>> = std::collections::HashMap::new();

    for (idx, &num) in nums.iter().enumerate() {
        positions.entry(num).or_default().push(idx);
    }

    for operation in operations {
        if let Some(old_num_positions) = positions.remove(&operation[0]) {
            for pos in &old_num_positions {
                nums[*pos] = operation[1];
            }
            positions
                .entry(operation[1])
                .or_default()
                .extend(old_num_positions);
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_change_basic() {
        assert_eq!(
            array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]),
            vec![3, 2, 7, 1]
        );
        assert_eq!(
            array_change(vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]]),
            vec![2, 1]
        )
    }
}
