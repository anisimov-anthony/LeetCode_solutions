#![allow(dead_code)]
pub fn get_maximum_generated(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut nums = vec![0; n as usize + 1];
    for i in 0..n {
        if i <= 1 {
            nums[i as usize] = i;
        }
        if 2 * i <= n && 2 <= 2 * i {
            nums[2 * i as usize] = nums[i as usize];
        }
        if 2 * i + 1 <= n && 2 <= 2 * i + 1 {
            nums[2 * i as usize + 1] = nums[i as usize] + nums[i as usize + 1]
        }
    }

    *nums.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_generated_basic() {
        assert_eq!(get_maximum_generated(7), 3);
        assert_eq!(get_maximum_generated(2), 1);
        assert_eq!(get_maximum_generated(3), 2);
    }
}
