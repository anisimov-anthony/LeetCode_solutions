#[allow(dead_code)]
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in nums.iter() {
        if num.to_string().len() % 2 == 0 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers_basic() {
        assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
