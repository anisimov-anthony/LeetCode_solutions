#![allow(dead_code)]
pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut history = std::collections::HashMap::new();
    for path in paths.iter() {
        for city in path.iter() {
            if history.contains_key(&city) {
                history.entry(city).and_modify(|ct| *ct -= 1);
            } else {
                history.entry(&city).or_insert(1);
            }
        }
    }
    let mut borders = Vec::with_capacity(2);
    for (city, counter) in history.iter() {
        if *counter == 1 {
            borders.push(city);
        }
    }

    for path in paths.iter() {
        if path[1] == **borders[0] {
            return borders[0].to_string();
        }
    }
    borders[1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dest_city_basic() {
        assert_eq!(
            dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo".to_string()
        );
        assert_eq!(
            dest_city(vec![
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string(), "B".to_string()],
                vec!["C".to_string(), "A".to_string()]
            ]),
            "A".to_string()
        );
        assert_eq!(
            dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
            "Z".to_string()
        );
    }
}
