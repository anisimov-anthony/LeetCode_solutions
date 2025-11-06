#![allow(dead_code)]
pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut wins = std::collections::HashMap::new();
    let mut loses = std::collections::HashMap::new();
    let mut teams = std::collections::HashSet::new();

    for mat in matches.iter() {
        wins.entry(mat[0]).and_modify(|fr| *fr += 1).or_insert(1);
        loses.entry(mat[1]).and_modify(|fr| *fr += 1).or_insert(1);
        teams.insert(mat[0]);
        teams.insert(mat[1]);
    }

    let mut first = Vec::new();
    let mut second = Vec::new();

    for &team in teams.iter() {
        match loses.get(&team) {
            Some(&ls) => {
                if ls == 1 {
                    second.push(team);
                }
            }
            None => {
                first.push(team);
            }
        }
    }

    first.sort_unstable();
    second.sort_unstable();

    vec![first, second]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winners_basic() {
        assert_eq!(
            find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
        assert_eq!(
            find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
