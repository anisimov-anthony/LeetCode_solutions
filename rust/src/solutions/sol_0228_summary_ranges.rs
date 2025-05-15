#[allow(dead_code)]
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    if nums.is_empty() {
        return result;
    }

    let mut first = nums[0];
    let mut second = nums[0];
    let mut rangenum;

    for i in 1..nums.len() {
        if nums[i] == second + 1 {
            second = nums[i];
        } else {
            if second == first {
                rangenum = second.to_string();
            } else {
                rangenum = first.to_string() + "->" + &second.to_string();
            }
            result.push(rangenum);
            second = nums[i];
            first = second;
        }
    }
    if second == first {
        rangenum = second.to_string();
    } else {
        rangenum = first.to_string() + "->" + &second.to_string();
    }
    result.push(rangenum);

    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_summary_ranges_basic() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]
        );

        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string()
            ]
        );
    }
}
