#![allow(dead_code)]
pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    if nums.chunks(3).all(|chnk| {
        chnk[0].abs_diff(chnk[1]) <= k as u32
            && chnk[0].abs_diff(chnk[2]) <= k as u32
            && chnk[1].abs_diff(chnk[2]) <= k as u32
    }) {
        return nums
            .chunks(3)
            .map(|chnk| chnk.to_vec())
            .collect::<Vec<Vec<i32>>>();
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_array_basic() {
        assert_eq!(
            divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        );
        assert_eq!(
            divide_array(vec![2, 4, 2, 2, 5, 2], 2),
            Vec::<Vec<i32>>::new()
        );

        assert_eq!(
            divide_array(
                vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11],
                14
            ),
            vec![
                vec![2, 2, 2],
                vec![4, 5, 5],
                vec![5, 5, 7],
                vec![7, 8, 8],
                vec![9, 9, 10],
                vec![11, 12, 12]
            ]
        )
    }
}
