#![allow(dead_code)]
pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
    let mut freqs = std::collections::HashMap::new();
    for resp in responses.iter() {
        let mut local_history = std::collections::HashSet::new();
        for rs in resp.iter() {
            if !local_history.contains(&rs) {
                local_history.insert(rs);
                freqs.entry(rs).and_modify(|fr| *fr += 1).or_insert(1);
            }
        }
    }
    let max_fr = freqs.iter().map(|fr| *fr.1).max().unwrap();
    let mut maxes: Vec<_> = freqs
        .iter()
        .filter(|fr| *fr.1 == max_fr)
        .map(|fr| *fr.0)
        .collect();
    maxes.sort();
    maxes[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_common_response_basic() {
        assert_eq!(
            find_common_response(vec![
                vec![
                    "good".to_string(),
                    "ok".to_string(),
                    "good".to_string(),
                    "ok".to_string()
                ],
                vec![
                    "ok".to_string(),
                    "bad".to_string(),
                    "good".to_string(),
                    "ok".to_string(),
                    "ok".to_string()
                ],
                vec!["good".to_string()],
                vec!["bad".to_string()]
            ]),
            "good"
        );
        assert_eq!(
            find_common_response(vec![
                vec!["good".to_string(), "ok".to_string(), "good".to_string()],
                vec!["ok".to_string(), "bad".to_string()],
                vec!["bad".to_string(), "notsure".to_string()],
                vec!["great".to_string(), "good".to_string()]
            ]),
            "bad"
        );
    }
}
