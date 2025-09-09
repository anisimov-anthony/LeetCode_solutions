#![allow(dead_code)]
pub fn count_largest_group(n: i32) -> i32 {
    fn sum_of_digits(mut num: i32) -> u32 {
        let mut sum = 0;
        while num > 0 {
            sum += (num % 10) as u32;
            num /= 10;
        }
        sum
    }

    let mut groups = std::collections::HashMap::new();
    for num in 1..=n {
        let sum = sum_of_digits(num);
        groups.entry(sum).or_insert_with(Vec::new).push(num);
    }

    let max_size = groups.iter().map(|(_, vc)| vc.len()).max().unwrap();
    groups.iter().filter(|(_, vc)| vc.len() == max_size).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_largest_group_basic() {
        assert_eq!(count_largest_group(13), 4);
        assert_eq!(count_largest_group(2), 2);
    }
}
