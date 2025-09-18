#![allow(dead_code)]
pub fn interpret(command: String) -> String {
    let mut res = String::new();
    let mut buff = String::new();
    for ch in command.chars() {
        if ch == 'G' {
            res.push_str(&"G".to_string());
        } else if ch == ')' {
            if buff.len() > 1 {
                res.push_str(&"al".to_string());
            } else {
                res.push_str(&"o".to_string());
            }
            buff.clear();
        } else {
            buff.push_str(&ch.to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_basic() {
        assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
        assert_eq!(
            interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
