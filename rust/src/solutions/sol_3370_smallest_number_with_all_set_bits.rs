#![allow(dead_code)]
pub fn smallest_number(n: i32) -> i32 {
    let mut n = n;
    loop {
        if (n + 1) & n == 0 {
            break;
        }
        n += 1;
    }
    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_number_basic() {
        assert_eq!(smallest_number(5), 7);
        assert_eq!(smallest_number(10), 15);
        assert_eq!(smallest_number(3), 3);
    }
}
