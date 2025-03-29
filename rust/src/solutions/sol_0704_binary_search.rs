#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if target < nums[mid] {
            right = mid;
        } else if target > nums[mid] {
            left = mid;
        } else {
            return mid as i32;
        }
    }

    if nums[left] == target {
        return left as i32;
    }
    if nums[right - 1] == target {
        return right as i32;
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_basic() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
