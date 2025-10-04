#![allow(dead_code)]
pub fn percentage_letter(s: String, letter: char) -> i32 {
    let counter = s.chars().filter(|ch| *ch == letter).count();
    (counter * 100 / s.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_letter_basic() {
        assert_eq!(percentage_letter("foobar".to_string(), 'o'), 33);
        assert_eq!(percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
