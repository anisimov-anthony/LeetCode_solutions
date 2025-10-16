#![allow(dead_code)]
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut converted = Vec::new();
    let mut ones_counter = 0;
    let mut was_zero = false;
    for nm in nums.iter() {
        if *nm == 0 {
            if !was_zero {
                converted.push(ones_counter);
            }

            converted.push(0);
            ones_counter = 0;
            was_zero = true;
        } else {
            ones_counter += 1;
            was_zero = false;
        }
    }
    if !was_zero {
        converted.push(ones_counter);
    } else {
        converted.push(0);
    }

    if converted.len() <= 2 {
        return 0.max(converted.iter().sum::<i32>() - 1);
    }

    converted
        .windows(3)
        .map(|cor| cor.iter().sum::<i32>())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray_basic() {
        assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
        assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
        assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    }
}
