#[allow(dead_code)]
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::<Vec<i32>>::new();

    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for interval in intervals {
        let mut need_add = true;
        for old_un in &mut result {
            if old_un[0] >= interval[0] && old_un[1] <= interval[1] {
                old_un[0] = interval[0];
                old_un[1] = interval[1];
                need_add = false;
                break;
            } else if old_un[0] >= interval[0]
                && (old_un[0] <= interval[1] && interval[1] <= old_un[1])
            {
                old_un[0] = interval[0];
                need_add = false;
                break;
            } else if old_un[1] <= interval[1]
                && (old_un[0] <= interval[0] && interval[0] <= old_un[1])
            {
                old_un[1] = interval[1];
                need_add = false;
                break;
            } else if old_un[0] <= interval[0] && old_un[1] >= interval[1] {
                need_add = false;
                break;
            }
        }
        if need_add {
            result.push(interval);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        let mut intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let mut valid = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(intervals), valid);

        intervals = vec![vec![1, 4], vec![4, 5]];
        valid = vec![vec![1, 5]];
        assert_eq!(merge(intervals), valid);
    }
}
