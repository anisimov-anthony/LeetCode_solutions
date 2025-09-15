#![allow(dead_code)]
pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for nm in nums1.iter() {
        if (nums2.contains(&nm) || nums3.contains(&nm)) && !res.contains(nm) {
            res.push(*nm);
        }
    }

    for nm in nums2.iter() {
        if (nums1.contains(&nm) || nums3.contains(&nm)) && !res.contains(nm) {
            res.push(*nm);
        }
    }

    for nm in nums3.iter() {
        if (nums1.contains(&nm) || nums2.contains(&nm)) && !res.contains(nm) {
            res.push(*nm);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_out_of_three_basic() {
        let mut output = two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]);
        let mut valid = vec![3, 2];
        output.sort();
        valid.sort();
        assert_eq!(output, valid);

        let mut output = two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]);
        let mut valid = vec![2, 3, 1];
        output.sort();
        valid.sort();
        assert_eq!(output, valid);

        let mut output = two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]);
        let mut valid = vec![];
        output.sort();
        valid.sort();

        assert_eq!(output, valid);
    }
}
