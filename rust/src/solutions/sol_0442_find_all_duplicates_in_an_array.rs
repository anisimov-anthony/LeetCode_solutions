#![allow(dead_code)]
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return Vec::new();
    }

    let mut result = Vec::new();
    fn counting_sort(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }

        let max = *nums.iter().max().unwrap() as usize;
        let mut count = vec![0; max + 1];

        for &num in &nums {
            count[num as usize] += 1;
        }

        let mut sorted = Vec::with_capacity(nums.len());
        for (num, &freq) in count.iter().enumerate() {
            for _ in 0..freq {
                sorted.push(num as i32);
            }
        }

        sorted
    }

    let sorted = counting_sort(nums);
    let mut uniq = sorted[0];
    for i in 1..sorted.len() {
        if sorted[i] == uniq {
            result.push(sorted[i]);
        } else {
            uniq = sorted[i];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_basic() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(find_duplicates(vec![1]), vec![]);
    }
}
