#[allow(dead_code)]
pub fn rotate_string(s: String, goal: String) -> bool {
    for i in 0..s.len() {
        let mut rt: Vec<char> = s.clone().chars().collect::<Vec<_>>();
        rt.rotate_left(i);
        if rt == goal.clone().chars().collect::<Vec<char>>() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string_basic() {
        assert!(rotate_string("abcde".to_string(), "cdeab".to_string()));
        assert!(!(rotate_string("abcde".to_string(), "abced".to_string())));
    }
}
