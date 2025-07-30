#![allow(dead_code)]
pub fn even_odd_bit(n: i32) -> Vec<i32> {
    let bits: Vec<_> = (0..32).map(|i| (n >> i) & 1).collect();
    let mut odd = 0;
    let mut even = 0;
    for (i, val) in bits.iter().enumerate() {
        if i % 2 != 0 {
            odd += val;
        } else {
            even += val;
        }
    }

    vec![even, odd]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd_bit_basic() {
        assert_eq!(even_odd_bit(50), vec![1, 2]);
        assert_eq!(even_odd_bit(2), vec![0, 1]);
    }
}
