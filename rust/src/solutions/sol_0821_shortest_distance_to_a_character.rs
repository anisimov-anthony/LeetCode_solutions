#![allow(dead_code)]
pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut result = Vec::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut positions = Vec::new();

    for (i, &ch) in chars.iter().enumerate() {
        if ch == c {
            positions.push(i);
        }
    }

    for (i, _) in chars.iter().enumerate() {
        let mut min_dist = i32::MAX;
        for &pos in &positions {
            let dist = (i as i32 - pos as i32).abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
        result.push(min_dist);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_to_char_basic() {
        assert_eq!(
            shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(shortest_to_char("aaab".to_string(), 'b'), vec![3, 2, 1, 0]);
    }
}
