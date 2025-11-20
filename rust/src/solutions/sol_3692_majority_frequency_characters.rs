#![allow(dead_code)]
pub fn majority_frequency_group(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut freq = std::collections::HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut group_size = std::collections::HashMap::new();
    let mut max_group_size = 0;

    for (&char_val, &count) in &freq {
        group_size
            .entry(count)
            .or_insert_with(std::collections::HashSet::new)
            .insert(char_val);
        max_group_size = max_group_size.max(group_size[&count].len());
    }

    let mut candidates = vec![];
    for (&k, chars) in &group_size {
        if chars.len() == max_group_size {
            candidates.push((k, chars));
        }
    }

    candidates.sort_by_key(|&(k, _)| std::cmp::Reverse(k));
    let best_chars = &candidates[0].1;

    let mut result: Vec<char> = best_chars.iter().copied().collect();
    result.sort();
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_frequency_group_basic() {
        assert_eq!(majority_frequency_group("aaabbbccdddde".to_string()), "ab");
        assert_eq!(majority_frequency_group("abcd".to_string()), "abcd");
        assert_eq!(majority_frequency_group("pfpfgi".to_string()), "fp");
    }
}
