#![allow(dead_code)]
pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut population = vec![0; 101];
    for log in logs.iter() {
        for i in (log[0] as usize - 1950)..(log[1] as usize - 1950) {
            population[i] = population[i] + 1;
        }
    }

    let max_population = population
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|max| max.1)
        .unwrap();
    let max_year = population
        .iter()
        .enumerate()
        .find(|(_, val)| *val == max_population)
        .unwrap()
        .0;

    1950 + max_year as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_population_basic() {
        assert_eq!(
            maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
            1993
        );
        assert_eq!(
            maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
            1960
        );
    }
}
