#![allow(dead_code)]
pub fn is_ugly(n: i32) -> bool {
    let mut n = n;
    if n == 0 {
        return false;
    }

    if n == 1 {
        return true;
    }

    let acceptable_factors: std::collections::HashSet<_> = vec![2, 3, 5].into_iter().collect();
    while n != 1 {
        let mut primed = false;
        for &fact in acceptable_factors.iter() {
            if n % fact == 0 {
                n /= fact;
                primed = true;
                break;
            }
        }
        if !primed {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ugly_basic() {
        assert!(is_ugly(6));
        assert!(is_ugly(1));
        assert!(!is_ugly(14));
    }
}
