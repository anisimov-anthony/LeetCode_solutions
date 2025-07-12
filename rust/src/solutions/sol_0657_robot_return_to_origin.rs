#[allow(dead_code)]
pub fn judge_circle(moves: String) -> bool {
    let mut us = 0;
    let mut ds = 0;
    let mut ls = 0;
    let mut rs = 0;
    for i in moves.chars() {
        match i {
            'U' => us += 1,
            'D' => ds += 1,
            'L' => ls += 1,
            'R' => rs += 1,
            _ => unreachable!(),
        }
    }

    (us == ds) && (ls == rs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_circle_basic() {
        assert_eq!(judge_circle("UD".to_string()), true);
        assert_eq!(judge_circle("LL".to_string()), false);
    }
}
