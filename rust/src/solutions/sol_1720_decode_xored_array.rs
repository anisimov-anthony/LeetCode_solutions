#![allow(dead_code)]
pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(encoded.len() + 1);
    res.push(first);
    let mut last = first;
    for enc in encoded.iter() {
        let new_dec = enc ^ last;
        last = new_dec;
        res.push(new_dec);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_basic() {
        assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
