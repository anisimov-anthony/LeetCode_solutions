#![allow(dead_code)]
pub fn total_money(n: i32) -> i32 {
    let full_weeks = n / 7;
    let remaining_days = n % 7;

    let full_weeks_sum = full_weeks * 28 + 7 * full_weeks * (full_weeks - 1) / 2;

    let mut remaining_sum = 0;
    for i in 0..remaining_days {
        remaining_sum += full_weeks + 1 + i;
    }

    full_weeks_sum + remaining_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_money_basic() {
        assert_eq!(total_money(4), 10);
        assert_eq!(total_money(10), 37);
        assert_eq!(total_money(20), 96);
    }
}
