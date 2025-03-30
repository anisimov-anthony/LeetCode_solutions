#[allow(dead_code)]
pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let chars3: Vec<char> = s3.chars().collect();

    let min_len = s1.len().min(s2.len().min(s3.len()));

    let mut common_prefix = String::new();
    for i in 0..min_len {
        if chars1[i] == chars2[i] && chars1[i] == chars3[i] {
            common_prefix.push(chars1[i]);
        } else {
            break;
        }
    }

    if common_prefix.len() == 0 {
        return -1;
    }

    (s1.len() + s2.len() + s3.len() - 3 * common_prefix.len()) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_minimum_operations_basic() {
        assert_eq!(
            find_minimum_operations("abc".to_string(), "abb".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            find_minimum_operations("dac".to_string(), "bac".to_string(), "cac".to_string()),
            -1
        );
    }
}
