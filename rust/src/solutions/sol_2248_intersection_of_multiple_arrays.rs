#![allow(dead_code)]
pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    if nums.len() == 1 {
        let mut result = nums[0].clone();
        result.sort();
        return result;
    }
    let mut nums = nums;
    nums.sort_by(|a, b| b.len().cmp(&a.len()));

    if nums[0].is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    for i in 0..nums[0].len() {
        let mut cont = true;
        for j in 1..nums.len() {
            if !nums[j].contains(&nums[0][i]) {
                cont = false;
                break;
            }
        }

        if cont {
            result.push(nums[0][i]);
        }
    }

    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_basic() {
        assert_eq!(
            intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            vec![3, 4]
        );
        assert_eq!(intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![]);
    }
}
