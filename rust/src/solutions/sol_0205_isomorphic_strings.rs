#![allow(dead_code)]
pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = std::collections::HashMap::new();
    let mut t_to_s = std::collections::HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        match (s_to_t.get(&sc), t_to_s.get(&tc)) {
            (Some(&mapped_tc), _) if mapped_tc != tc => return false,
            (_, Some(&mapped_sc)) if mapped_sc != sc => return false,
            _ => {
                s_to_t.insert(sc, tc);
                t_to_s.insert(tc, sc);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic_basic() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
    }
}
