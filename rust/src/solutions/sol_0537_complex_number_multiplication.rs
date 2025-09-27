#![allow(dead_code)]
pub fn complex_number_multiply(num1: String, num2: String) -> String {
    fn re_im_parts(num: String) -> (i32, i32) {
        let splitted = num.split('+');
        let splitted = splitted.collect::<Vec<&str>>();
        let real = splitted[0].parse::<i32>().unwrap();
        let mut im = splitted[1].to_string();
        im.pop();
        let imagine = im.parse::<i32>().unwrap();
        (real, imagine)
    }

    let nm1 = re_im_parts(num1);
    let nm2 = re_im_parts(num2);
    let real = nm1.0 * nm2.0 - nm1.1 * nm2.1;
    let imagine = nm1.0 * nm2.1 + nm2.0 * nm1.1;
    format!("{}+{}i", real, imagine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_number_multiply_basic() {
        assert_eq!(
            complex_number_multiply("1+1i".to_string(), "1+1i".to_string()),
            "0+2i"
        );
        assert_eq!(
            complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()),
            "0+-2i"
        );
    }
}
