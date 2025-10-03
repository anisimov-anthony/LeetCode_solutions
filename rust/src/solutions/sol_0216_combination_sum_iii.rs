#![allow(dead_code)]
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    (0..1 << 9).fold(vec![], |mut a, x| {
        let b = (0..=8).fold(vec![], |mut b, y| {
            if (x >> y) & 1 == 1 {
                b.push(y + 1);
            }
            b
        });
        if b.len() == k as usize && b.iter().sum::<i32>() == n {
            a.push(b);
        }
        a
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum3_basic() {
        assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(
            combination_sum3(3, 9),
            vec![vec![2, 3, 4], vec![1, 3, 5], vec![1, 2, 6]]
        );
    }
}
