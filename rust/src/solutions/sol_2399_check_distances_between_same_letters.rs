#![allow(dead_code)]
pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut occurences = std::collections::HashMap::new();
    for (i, ch) in s.chars().enumerate() {
        occurences.entry(ch).or_insert(Vec::new()).push(i);
    }

    for (&ch, idxes) in occurences.iter() {
        if distance[(ch as u8 - b'a') as usize] != idxes[0].abs_diff(idxes[1]) as i32 - 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_distances_basic() {
        assert!(check_distances(
            "abaccb".to_string(),
            vec![
                1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));

        assert!(!check_distances(
            "aa".to_string(),
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));
    }
}
