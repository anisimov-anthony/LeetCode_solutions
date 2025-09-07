#![allow(dead_code)]
pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut freqs = std::collections::HashMap::new();
    for elem in arr.iter() {
        freqs.entry(elem).and_modify(|fr| *fr += 1).or_insert(1);
    }

    let distincts: Vec<&String> = arr
        .iter()
        .filter(|elem| *freqs.get(elem).unwrap() == 1)
        .collect();

    if (k as usize) <= distincts.len() {
        return distincts[(k - 1) as usize].to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_distinct_basic() {
        assert_eq!(
            kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a".to_string()
        );
        assert_eq!(
            kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
                1
            ),
            "aaa".to_string()
        );
        assert_eq!(
            kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3),
            String::new()
        );
    }
}
