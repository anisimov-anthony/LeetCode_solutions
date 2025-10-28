#![allow(dead_code)]
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut result = 0;
    for wd_size in 3..=nums.len() {
        result += nums
            .windows(wd_size)
            .filter(|wd| {
                let diff = wd[1] - wd[0];
                wd.windows(2).all(|w| w[1] - w[0] == diff)
            })
            .count()
    }

    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arithmetic_slices_basic() {
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(number_of_arithmetic_slices(vec![1]), 0);
    }
}
