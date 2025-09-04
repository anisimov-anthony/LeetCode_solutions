#![allow(dead_code)]
pub fn trim_mean(arr: Vec<i32>) -> f64 {
    let mut arr = arr;
    arr.sort();

    let p5 = arr.len() * 5 / 100;

    arr[(p5 as usize)..=(arr.len() - p5 as usize - 1)]
        .iter()
        .sum::<i32>() as f64
        / (arr.len() - 2 * p5 as usize) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_mean_basic() {
        assert_eq!(
            trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.0f64
        );
        assert_eq!(
            trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ]),
            4.0f64
        );
        assert_eq!(
            trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
            ]),
            4.777777777777778f64
        );
    }
}
