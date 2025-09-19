#![allow(dead_code)]
pub fn sort_sentence(s: String) -> String {
    let words: Vec<_> = s.split_whitespace().collect();

    let mut res = vec![" ".to_string(); words.len()];
    for word in words.iter() {
        let idx = word.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;
        let target = word.chars().take(word.len() - 1).collect::<String>();
        res[idx] = target;
    }
    res.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_sentence_basic() {
        assert_eq!(
            sort_sentence("is2 sentence4 This1 a3".to_string()),
            "This is a sentence"
        );
        assert_eq!(
            sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I".to_string()
        );
    }
}
