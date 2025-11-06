#![allow(dead_code)]
pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut n = n;
    let mut bits = Vec::new();

    while n > 0 {
        bits.push((n & 1) as u8);
        n >>= 1;
    }

    bits.reverse();

    let _ = bits.iter_mut().for_each(|dg| *dg = 1 - *dg);

    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + (*val as i32) * 2i32.pow(i as u32)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_complement_basic() {
        assert_eq!(bitwise_complement(5), 2);
        assert_eq!(bitwise_complement(7), 0);
        assert_eq!(bitwise_complement(10), 5);
    }
}
