#![allow(dead_code)]
pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![celsius + 273.15, celsius * 1.80 + 32.00]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_temperature_basic() {
        assert_eq!(convert_temperature(36.50), vec![309.65000, 97.70000]);
        assert_eq!(convert_temperature(122.11), vec![395.26000, 251.79800]);
    }
}
