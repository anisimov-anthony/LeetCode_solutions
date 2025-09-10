#![allow(dead_code)]
pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    nums.windows(2)
        .step_by(2)
        .for_each(|pair| res.extend(vec![pair[1]; pair[0] as usize]));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_rl_elist_basic() {
        assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
        assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
    }
}
