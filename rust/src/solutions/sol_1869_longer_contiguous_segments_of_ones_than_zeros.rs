#![allow(dead_code)]
pub fn check_zero_ones(s: String) -> bool {
    let mut last_elem: i32 = -1;
    let mut o_size = 0;
    let mut z_size = 0;
    let mut local_counter = 0;

    for ch in s.chars() {
        if ch.to_digit(10).unwrap() as i32 != last_elem {
            if last_elem == 1 {
                o_size = o_size.max(local_counter);
            } else {
                z_size = z_size.max(local_counter);
            }
            last_elem = ch.to_digit(10).unwrap() as i32;
            local_counter = 0;
        }
        local_counter += 1;
    }
    if last_elem == 1 {
        o_size = o_size.max(local_counter);
    } else {
        z_size = z_size.max(local_counter);
    }

    o_size > z_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_zero_ones_basic() {
        assert!(check_zero_ones("1101".to_string()));
        assert!(!check_zero_ones("111000".to_string()));
        assert!(!check_zero_ones("110100010".to_string()));
    }
}
