#[allow(dead_code)]
pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .chars()
        .map(|x| x as i32 - 'A' as i32 + 1)
        .fold(0, |prev, next| prev * 26 + next)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_title_to_number_basic() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }
}
