#![allow(dead_code)]
pub fn check_if_pangram(sentence: String) -> bool {
    let mut occurences = vec![-1; 26];
    for sn in sentence.chars() {
        occurences[(sn as u8 - 97) as usize] = 1;
    }

    occurences.iter().all(|occ| *occ == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_pangram_basic() {
        assert!(check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
        assert!(!check_if_pangram("leetcode".to_string()));
    }
}
