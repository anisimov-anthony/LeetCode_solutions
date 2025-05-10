use std::usize;

#[allow(dead_code)]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freqs = std::collections::HashMap::new();
    for &num in &nums {
        match freqs.get_mut(&num) {
            Some(fr) => *fr += 1,
            None => {
                freqs.insert(num, 1);
            }
        }
    }

    let mut ks = vec![Vec::new(); nums.len() + 1];
    for (num, freq) in freqs {
        ks[freq as usize].push(num);
    }

    let mut result = Vec::new();
    for i in (0..=nums.len()).rev() {
        if !ks[i].is_empty() {
            result.extend(&ks[i]);
            if result.len() >= k as usize {
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_basic() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
