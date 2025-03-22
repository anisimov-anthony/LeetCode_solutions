#[allow(dead_code)]
pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    // corner case
    if nums.is_empty() {
        return 0;
    }

    // main case
    let mut llc = 1;
    let mut result = 1;
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            llc += 1;
            result = result.max(llc);
        } else {
            llc = 1;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_find_length_of_lcis_basic() {
        assert_eq!(find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1)
    }
}
