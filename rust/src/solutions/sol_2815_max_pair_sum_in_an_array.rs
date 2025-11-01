#![allow(dead_code)]
pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut groups = std::collections::HashMap::new();
    for num in nums.iter() {
        let max_digit = num.to_string().chars().max_by(|&a, &b| a.cmp(&b));

        groups.entry(max_digit).or_insert_with(Vec::new).push(num);
    }

    let mut result = -1;
    for (_, values) in groups.iter_mut() {
        values.sort_unstable();
        if values.len() != 1 {
            let first = values[values.len() - 1];
            let second = values[values.len() - 2];

            result = result.max(*first + *second)
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_basic() {
        assert_eq!(max_sum(vec![112, 131, 411]), -1);
        assert_eq!(max_sum(vec![2536, 1613, 3366, 162]), 5902);
        assert_eq!(max_sum(vec![51, 71, 17, 24, 42]), 88);
    }
}
