#![allow(dead_code)]
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let indices = match k {
        0.. => 1..=k,
        _ => k..=-1,
    };
    (0..code.len() as i32)
        .map(|i| {
            indices
                .clone()
                .map(|di| (i + di + code.len() as i32) % code.len() as i32)
                .map(|i| code[i as usize])
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
