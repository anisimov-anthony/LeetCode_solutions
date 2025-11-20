#![allow(dead_code)]
pub fn check_ones_segment(s: String) -> bool {
    let mut segments = 0;
    let mut in_segment = false;

    for ch in s.chars() {
        if ch == '1' {
            if !in_segment {
                segments += 1;
                in_segment = true;
            }
        } else {
            in_segment = false;
        }
    }

    segments <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ones_segment_basic() {
        assert!(!check_ones_segment("1001".to_string()));
        assert!(check_ones_segment("110".to_string()));
    }
}
