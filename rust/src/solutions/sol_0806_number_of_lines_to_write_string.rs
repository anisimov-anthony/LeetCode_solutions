#![allow(dead_code)]
pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 0;
    let mut last_widtgh = 0;
    for smb in s.chars() {
        if last_widtgh + widths[(smb as u8 - b'a') as usize] <= 100 {
            last_widtgh += widths[(smb as u8 - b'a') as usize];
        } else {
            last_widtgh = widths[(smb as u8 - b'a') as usize];
            lines += 1;
        }
    }
    if last_widtgh != 0 {
        lines += 1;
    }
    vec![lines, last_widtgh]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_lines_basic() {
        assert_eq!(
            number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            vec![3, 60]
        );
        assert_eq!(
            number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}
