#[allow(dead_code)]
pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut result = String::with_capacity(s.len() + spaces.len());
    let mut first = 0;
    let mut second = 0;

    for c in s.chars() {
        if first != spaces[second] {
            result.push(c);
            first += 1;
        } else {
            result.push_str(" ");
            if second != spaces.len() - 1 {
                second += 1;
            }
            result.push(c);
            first += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_spaces_basic() {
        assert_eq!(
            add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn".to_string()
        );
        assert_eq!(
            add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9]),
            "i code in py thon".to_string()
        );
        assert_eq!(
            add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g".to_string()
        );
    }
}
