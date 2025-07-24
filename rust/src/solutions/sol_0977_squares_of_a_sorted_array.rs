#[allow(dead_code)]
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // corner cases
    if nums.is_empty() {
        return nums;
    }
    if nums.len() == 1 {
        let mut result = Vec::new();
        result.push(nums[0].pow(2));
        return result;
    }

    // main case
    let mut negs = Vec::new();
    for &num in nums.iter() {
        if num >= 0 {
            break;
        }
        negs.push(num);
    }

    let mut negs: Vec<i32> = negs.iter_mut().map(|ng| ng.pow(2)).collect();
    negs.sort();

    let mut poss = Vec::new();
    for &num in nums.iter() {
        if num >= 0 {
            poss.push(num);
        }
    }

    let poss: Vec<i32> = poss.iter_mut().map(|ps| ps.pow(2)).collect();

    let mut merged = Vec::with_capacity(negs.len() + poss.len());
    let (mut i, mut j) = (0, 0);

    while i < negs.len() && j < poss.len() {
        if negs[i] < poss[j] {
            merged.push(negs[i]);
            i += 1;
        } else {
            merged.push(poss[j]);
            j += 1;
        }
    }

    merged.extend(&negs[i..]);
    merged.extend(&poss[j..]);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares_basic() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );

        assert_eq!(
            sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
