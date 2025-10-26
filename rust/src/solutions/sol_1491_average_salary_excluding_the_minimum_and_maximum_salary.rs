#![allow(dead_code)]
pub fn average(salary: Vec<i32>) -> f64 {
    ((salary.iter().sum::<i32>() - salary.iter().max().unwrap() - salary.iter().min().unwrap())
        as f64)
        / ((salary.len() - 2) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_basic() {
        assert_eq!(average(vec![4000, 3000, 1000, 2000]), 2500.00f64);
        assert_eq!(average(vec![1000, 2000, 3000]), 2000.00f64);
    }
}
