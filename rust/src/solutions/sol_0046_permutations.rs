#[allow(dead_code)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }

    fn factorial(i: usize) -> usize {
        if i == 0 {
            return 1;
        }
        factorial(i - 1) * i
    }

    let mut levels = std::collections::HashMap::new();

    fn cyclic_insert_element_in_sequence(sequence: Vec<i32>, element: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(sequence.len() + 1);
        for i in 0..=sequence.len() {
            let mut tmp_sequence = sequence.clone();
            tmp_sequence.insert(i, element);
            result.push(tmp_sequence);
        }
        result
    }

    levels.insert(0, vec![vec![nums[0]]]);

    for i in 1..nums.len() {
        let prev_level = levels.get(&(i - 1)).unwrap().clone();
        let mut new_level = Vec::with_capacity(factorial(i + 1));

        for sub_level in &prev_level {
            let new_permutations = cyclic_insert_element_in_sequence(sub_level.clone(), nums[i]);
            new_level.extend(new_permutations);
        }

        levels.insert(i, new_level);
    }

    levels.get(&(nums.len() - 1)).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_basic() {
        let mut result = permute(vec![1, 2, 3]);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
        assert_eq!(permute(vec![0, 1]), vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(permute(vec![1]), vec![vec![1]]);
        assert_eq!(permute(vec![]), vec![] as Vec<Vec<i32>>);
    }
}
