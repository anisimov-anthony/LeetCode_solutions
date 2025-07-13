#[allow(dead_code)]
pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defang_i_paddr_basic() {
        assert_eq!(
            defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
        assert_eq!(
            defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );
    }
}
