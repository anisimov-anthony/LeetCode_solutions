#[allow(dead_code)]
pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut counter = 0;
    let mut can_type = true;

    for c in text.chars() {
        if c == ' ' {
            if can_type {
                counter += 1;
            } else {
                can_type = true;
            }
        } else {
            if broken_letters.contains(c) {
                can_type = false;
            }
        }
    }

    if can_type {
        counter += 1;
    }

    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_typed_words_basic() {
        assert_eq!(
            can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "e".to_string()),
            0
        );
    }
}
