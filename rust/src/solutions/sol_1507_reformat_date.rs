#![allow(dead_code)]
pub fn reformat_date(date: String) -> String {
    let ymd: Vec<_> = date.split_whitespace().collect();
    let y = ymd[2];
    let months = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let m = months.iter().position(|&el| el == ymd[1]).unwrap() + 1;
    let m = if m < 10 {
        "0".to_string() + &m.to_string()
    } else {
        m.to_string()
    };

    let d = &ymd[0][0..ymd[0].len() - 2]
        .to_string()
        .parse::<i32>()
        .unwrap();
    let d = if *d < 10 {
        "0".to_string() + &d.to_string()
    } else {
        d.to_string()
    };

    format!("{y}-{m}-{d}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reformat_date_basic() {
        assert_eq!(reformat_date("20th Oct 2052".to_string()), "2052-10-20");
        assert_eq!(reformat_date("6th Jun 1933".to_string()), "1933-06-06");
        assert_eq!(reformat_date("26th May 1960".to_string()), "1960-05-26");
    }
}
