#![allow(dead_code)]
pub fn min_time_to_type(word: String) -> i32 {
    let mut cur = 'a';
    let mut counter = 0;
    for ch in word.chars() {
        if ch != cur {
            let cl = i32::abs(ch as i32 - cur as i32);
            let cntcl = 26 - cl;
            counter += cl.min(cntcl);
            cur = ch;
        }

        counter += 1;
    }

    counter as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_to_type_basic() {
        assert_eq!(min_time_to_type("abc".to_string()), 5);
        assert_eq!(min_time_to_type("bza".to_string()), 7);
        assert_eq!(min_time_to_type("zjpc".to_string()), 34);
    }
}
