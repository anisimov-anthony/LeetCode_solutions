#[allow(dead_code)]
pub fn find_complement(num: i32) -> i32 {
    let mask = (1 << (32 - num.leading_zeros())) - 1;
    !num & mask
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_complement_basic() {
        assert_eq!(find_complement(5), 2);
        assert_eq!(find_complement(1), 0);
    }
}
