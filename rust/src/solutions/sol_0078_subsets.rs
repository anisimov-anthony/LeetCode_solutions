#[allow(dead_code)]
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());

        for i in start..nums.len() {
            current.push(nums[i]);

            backtrack(nums, i + 1, current, result);

            current.pop();
        }
    }

    backtrack(&nums, 0, &mut current, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let result1 = subsets(vec![1, 2, 3]);
        let expected1 = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ];
        assert_eq!(result1.len(), expected1.len());
        for subset in &result1 {
            assert!(expected1.contains(subset));
        }

        let result2 = subsets(vec![0]);
        let expected2 = vec![vec![], vec![0]];
        assert_eq!(result2.len(), expected2.len());
        for subset in &result2 {
            assert!(expected2.contains(subset));
        }

        let result3 = subsets(vec![]);
        let expected3 = vec![vec![]];
        assert_eq!(result3, expected3);
    }
}
