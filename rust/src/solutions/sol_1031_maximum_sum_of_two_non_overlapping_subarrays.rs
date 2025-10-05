#![allow(dead_code)]
pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for i in 0..=(nums.len() - first_len as usize) {
        let lsum = nums[i..i + first_len as usize].iter().sum::<i32>();
        first.push(vec![lsum, i as i32, i as i32 + first_len - 1]);
    }
    for i in 0..=(nums.len() - second_len as usize) {
        let lsum = nums[i..i + second_len as usize].iter().sum::<i32>();
        second.push(vec![lsum, i as i32, i as i32 + second_len - 1]);
    }
    let mut max_sum = 0;

    for fs in first.iter() {
        for sn in second.iter() {
            if (fs[2] < sn[1]) || (sn[2] < fs[1]) {
                let current_sum = fs[0] + sn[0];
                if current_sum > max_sum {
                    max_sum = current_sum;
                }
            }
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_two_no_overlap_basic() {
        assert_eq!(
            max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
            20
        );
        assert_eq!(
            max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
            29
        );
        assert_eq!(
            max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
            31
        );
    }
}
