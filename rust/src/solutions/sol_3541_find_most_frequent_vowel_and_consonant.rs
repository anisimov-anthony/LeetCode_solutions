#![allow(dead_code)]
pub fn max_freq_sum(s: String) -> i32 {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut v_freqs = std::collections::HashMap::new();
    let mut c_freqs = std::collections::HashMap::new();
    for ch in s.chars() {
        if vowels.contains(&ch) {
            v_freqs.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
        } else {
            c_freqs.entry(ch).and_modify(|fr| *fr += 1).or_insert(1);
        }
    }

    let v_max = v_freqs.iter().map(|ent| ent.1).max().unwrap_or(&0);
    let c_max = c_freqs.iter().map(|ent| ent.1).max().unwrap_or(&0);
    v_max + c_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_freq_sum_basic() {
        assert_eq!(max_freq_sum("successes".to_string()), 6);
        assert_eq!(max_freq_sum("aeiaeia".to_string()), 3);
    }
}
