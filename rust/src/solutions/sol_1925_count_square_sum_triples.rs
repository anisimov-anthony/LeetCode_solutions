#![allow(dead_code)]
pub fn count_triples(n: i32) -> i32 {
    let mut res = 0;
    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    res += 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_triples_basic() {
        assert_eq!(count_triples(5), 2);
        assert_eq!(count_triples(10), 4);
    }
}
