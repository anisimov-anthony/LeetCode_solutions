#![allow(dead_code)]
pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    let first = event1[0].clone().max(event2[0].clone());
    let second = event1[1].clone().min(event2[1].clone());

    first <= second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_have_conflict_basic() {
        assert!(have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        ));
        assert!(have_conflict(
            vec!["01:00".to_string(), "02:00".to_string()],
            vec!["01:20".to_string(), "03:00".to_string()]
        ));
        assert!(!have_conflict(
            vec!["10:00".to_string(), "11:00".to_string()],
            vec!["14:00".to_string(), "15:00".to_string()]
        ));
    }
}
