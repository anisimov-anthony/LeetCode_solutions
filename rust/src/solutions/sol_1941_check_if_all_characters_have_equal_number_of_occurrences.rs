#![allow(dead_code)]
pub fn are_occurrences_equal(s: String) -> bool {
    let mut freqs = std::collections::HashMap::new();
    for ch in s.chars() {
        freqs.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let ethal_fr = freqs.get(&s.chars().next().unwrap()).unwrap();
    for (_, fr) in freqs.iter() {
        if fr != ethal_fr {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_occurrences_equal_basic() {
        assert!(are_occurrences_equal("abacbc".to_string()));
        assert!(!are_occurrences_equal("aaabb".to_string()));
    }
}
