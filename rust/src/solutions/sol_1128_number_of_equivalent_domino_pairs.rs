#![allow(dead_code)]
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut uniques = std::collections::HashMap::new();
    let mut counter = 0;
    for dm in dominoes.iter() {
        let mut sel = dm.clone();
        sel.sort();

        uniques.entry(sel).and_modify(|fr| *fr += 1).or_insert(1);
    }

    for (_, fr) in uniques.iter() {
        if *fr > 1 {
            counter += (fr * (fr - 1)) / 2;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_equiv_domino_pairs_basic() {
        assert_eq!(
            num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
        assert_eq!(
            num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ]),
            3
        );
    }
}
