#[allow(dead_code)]
pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut ns = nums.clone();
    ns.sort();
    let mut result = 0;
    for pair in ns.chunks(2) {
        result += pair[0].min(pair[1]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_pair_sum_basic() {
        assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
        assert_eq!(array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
