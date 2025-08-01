#![allow(dead_code)]
pub fn is_strictly_palindromic(n: i32) -> bool {
    for i in 2..=(n - 2) {
        let based = format_radix(n as u32, i as u32);
        if !is_palindrome(based) {
            return false;
        }
    }

    fn is_palindrome(s: String) -> bool {
        s.chars().eq(s.chars().rev())
    }

    fn format_radix(mut x: u32, radix: u32) -> String {
        let mut result = vec![];

        loop {
            let m = x % radix;
            x = x / radix;

            // will panic if you use a bad radix (< 2 or > 36).
            result.push(std::char::from_digit(m, radix).unwrap());
            if x == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strictly_palindromic_basic() {
        assert!(!is_strictly_palindromic(9));
        assert!(!is_strictly_palindromic(4));
    }
}
