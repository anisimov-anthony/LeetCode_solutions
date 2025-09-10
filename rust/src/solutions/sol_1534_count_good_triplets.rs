#![allow(dead_code)]
pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut counter = 0;
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            for k in j + 1..arr.len() {
                if arr[i].abs_diff(arr[j]) <= a as u32
                    && arr[j].abs_diff(arr[k]) <= b as u32
                    && arr[i].abs_diff(arr[k]) <= c as u32
                {
                    counter += 1;
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_triplets_basic() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
