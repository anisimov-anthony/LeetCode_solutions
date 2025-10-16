#![allow(dead_code)]
pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if (original.len() as i32) != m * n {
        return vec![];
    }

    let mut res = Vec::with_capacity(m as usize);
    original
        .chunks(n as usize)
        .for_each(|chnk| res.push(chnk.to_vec()));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct2_d_array_basic() {
        assert_eq!(
            construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
        assert_eq!(construct2_d_array(vec![1, 2, 3], 1, 3), vec![vec![1, 2, 3]]);
        assert_eq!(construct2_d_array(vec![1, 2], 1, 1), Vec::<Vec<i32>>::new());
    }
}
