#![allow(dead_code)]
pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut assosiation = names
        .into_iter()
        .zip(heights)
        .collect::<Vec<(String, i32)>>();

    assosiation.sort_by(|a, b| b.1.cmp(&a.1));
    assosiation
        .iter()
        .map(|ass| ass.clone().0)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_people_basic() {
        assert_eq!(
            sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
        assert_eq!(
            sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
