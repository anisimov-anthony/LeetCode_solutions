#[allow(dead_code)]
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::from([-1, -1]);
    let first_hit = search(nums.clone(), target);
    if first_hit == -1 {
        return result;
    }

    let (mut left_bound, mut right_bound) = (first_hit, first_hit);

    while left_bound != -1 {
        result[0] = left_bound;
        left_bound = search((&nums[0..left_bound as usize]).to_vec(), target);
    }

    result[1] = right_bound;
    while right_bound != -1 {
        let next_right_bound = if (right_bound as usize + 1) < nums.len() {
            search((&nums[(right_bound as usize + 1)..]).to_vec(), target)
        } else {
            -1
        };
        if next_right_bound == -1 {
            break;
        }

        right_bound = right_bound + next_right_bound + 1;
        result[1] = right_bound;
    }

    result
}

#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left != right {
        let mid = left + (right - left) / 2;
        if target < nums[mid] {
            right = mid;
        } else if target > nums[mid] {
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }

    if left < nums.len() && nums[left] == target {
        return left as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range_basic() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
    }
}
