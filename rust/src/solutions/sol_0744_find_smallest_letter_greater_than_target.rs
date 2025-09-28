#![allow(dead_code)]
pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    *letters
        .iter()
        .filter(|f| f > &&target)
        .min()
        .unwrap_or(&letters[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter_basic() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
        assert_eq!(next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'), 'x');
    }
}
