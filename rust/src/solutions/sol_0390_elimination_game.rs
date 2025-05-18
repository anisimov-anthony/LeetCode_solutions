#[allow(dead_code)]
pub fn last_remaining(n: i32) -> i32 {
    let mut head = 1;
    let mut step = 1;
    let mut left = true;
    let mut others = n;
    while others > 1 {
        if left || others % 2 == 1 {
            head += step;
        }
        step *= 2;
        others /= 2;
        left = !left;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_remaining_basic() {
        assert_eq!(last_remaining(9), 6);
        assert_eq!(last_remaining(1), 1);
    }
}
