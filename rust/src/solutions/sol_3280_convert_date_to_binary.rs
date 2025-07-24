#![allow(dead_code)]
pub fn convert_date_to_binary(date: String) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return String::new();
    }

    let year = parts[0].parse::<u32>().unwrap_or(0);
    let month = parts[1].parse::<u32>().unwrap_or(0);
    let day = parts[2].parse::<u32>().unwrap_or(0);

    let year_bin = format!("{:b}", year);
    let month_bin = format!("{:b}", month);
    let day_bin = format!("{:b}", day);

    format!("{}-{}-{}", year_bin, month_bin, day_bin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_date_to_binary_basic() {
        assert_eq!(
            convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101".to_string()
        );
        assert_eq!(
            convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1".to_string()
        );
    }
}
