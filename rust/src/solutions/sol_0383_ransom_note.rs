#![allow(dead_code)]
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut rn = std::collections::HashMap::new();
    let mut mg = std::collections::HashMap::new();

    for r in ransom_note.chars() {
        rn.entry(r).and_modify(|f| *f += 1).or_insert(1);
    }

    for m in magazine.chars() {
        mg.entry(m).and_modify(|f| *f += 1).or_insert(1);
    }

    for r in rn.iter() {
        match mg.get(&r.0) {
            Some(value) => {
                if value < r.1 {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct_basic() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
        assert!(!can_construct("aa".to_string(), "ab".to_string()));
        assert!(can_construct("aa".to_string(), "aab".to_string()));
    }
}
