#![allow(dead_code)]
pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut ev = Vec::with_capacity((nums.len() + 1) / 2);
    let mut od = Vec::with_capacity(nums.len() / 2);

    for (i, &num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            ev.push(num);
        } else {
            od.push(num);
        }
    }

    ev.sort_unstable();
    od.sort_unstable_by(|a, b| b.cmp(a));

    let mut res = vec![0; nums.len()];

    for (i, &num) in ev.iter().enumerate() {
        res[i * 2] = num;
    }

    for (i, &num) in od.iter().enumerate() {
        res[i * 2 + 1] = num;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_even_odd_basic() {
        assert_eq!(sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
        assert_eq!(sort_even_odd(vec![2, 1]), vec![2, 1]);
    }
}
