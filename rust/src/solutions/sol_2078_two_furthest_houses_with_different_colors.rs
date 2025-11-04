#![allow(dead_code)]
pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..colors.len() {
        for j in 0..colors.len() {
            if i != j && colors[i] != colors[j] {
                result = result.max(i.abs_diff(j));
            }
        }
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance_basic() {
        assert_eq!(max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
        assert_eq!(max_distance(vec![1, 8, 3, 8, 3]), 4);
    }
}
