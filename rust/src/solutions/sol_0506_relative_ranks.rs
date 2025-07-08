#[allow(dead_code)]
pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut tops = score.clone();
    tops.sort_by(|a, b| b.cmp(a));
    let mut result = Vec::new();
    for sc in score.iter() {
        let pos = tops.iter().position(|x| x == sc).unwrap();
        match pos {
            0 => result.push("Gold Medal".to_string()),
            1 => result.push("Silver Medal".to_string()),
            2 => result.push("Bronze Medal".to_string()),
            _ => {
                let place = pos + 1;
                result.push(place.to_string());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_relative_ranks_basic() {
        assert_eq!(
            find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
        assert_eq!(
            find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec![
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string()
            ]
        )
    }
}
