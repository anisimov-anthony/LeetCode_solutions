#![allow(dead_code)]
pub fn square_is_white(coordinates: String) -> bool {
    let evens: std::collections::HashSet<_> = vec![
        "a".to_string(),
        "c".to_string(),
        "e".to_string(),
        "g".to_string(),
    ]
    .into_iter()
    .collect();
    let odds: std::collections::HashSet<_> = vec![
        "b".to_string(),
        "d".to_string(),
        "f".to_string(),
        "h".to_string(),
    ]
    .into_iter()
    .collect();

    let x = coordinates.chars().nth(0).unwrap().to_string();
    let y = coordinates
        .chars()
        .nth(1)
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap();

    if (evens.contains(&x) && (y % 2 == 1)) || (odds.contains(&x) && (y % 2 == 0)) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_is_white_basic() {
        assert!(!square_is_white("a1".to_string()));
        assert!(square_is_white("h3".to_string()));
        assert!(!square_is_white("c7".to_string()));
    }
}
