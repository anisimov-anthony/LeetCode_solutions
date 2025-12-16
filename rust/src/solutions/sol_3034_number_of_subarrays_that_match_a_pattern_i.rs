#![allow(dead_code)]
pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = pattern.len();
    let subarray_len = m + 1;

    if n < subarray_len {
        return 0;
    }

    let mut count = 0;

    for i in 0..=(n - subarray_len) {
        let mut matches = true;

        for k in 0..m {
            let curr = nums[i + k];
            let next = nums[i + k + 1];

            match pattern[k] {
                1 => {
                    if next <= curr {
                        matches = false;
                        break;
                    }
                }
                0 => {
                    if next != curr {
                        matches = false;
                        break;
                    }
                }
                -1 => {
                    if next >= curr {
                        matches = false;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }

        if matches {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matching_subarrays_basic() {
        assert_eq!(
            count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
        assert_eq!(
            count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
