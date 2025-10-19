#![allow(dead_code)]
pub fn are_numbers_ascending(s: String) -> bool {
    let tokens: Vec<_> = s.split_whitespace().collect();
    tokens
        .iter()
        .filter(|tk| tk.chars().all(|ch| ch.is_digit(10)))
        .map(|fl| fl.parse::<i32>().unwrap())
        .is_sorted_by(|a, b| a.lt(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_numbers_ascending_basic() {
        assert!(are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ));
        assert!(!are_numbers_ascending("hello world 5 x 5".to_string()));
        assert!(!are_numbers_ascending(
            "unset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ));
    }
}
