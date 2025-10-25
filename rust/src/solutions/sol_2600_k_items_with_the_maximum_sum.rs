#![allow(dead_code)]
pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, _num_neg_ones: i32, k: i32) -> i32 {
    num_ones.min(k) - (k - num_ones - num_zeros).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_items_with_maximum_sum_basic() {
        assert_eq!(k_items_with_maximum_sum(3, 2, 0, 2), 2);
        assert_eq!(k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }
}
