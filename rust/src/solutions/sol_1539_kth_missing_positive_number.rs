#![allow(dead_code)]
pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut missing = 0;
    let mut num = 1;
    let mut idx = 0;

    while missing < k {
        if idx < arr.len() && arr[idx] == num {
            idx += 1;
        } else {
            missing += 1;
            if missing == k {
                return num;
            }
        }
        num += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_positive_basic() {
        assert_eq!(find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
