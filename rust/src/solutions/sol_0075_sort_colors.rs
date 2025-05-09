#[allow(dead_code)]
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut step = nums.len();
    let mut flag = false;
    while step > 1 || flag {
        if step > 1 {
            step = step * 4 / 5;
        }
        flag = false;
        let mut i = 0;
        while i + step < nums.len() {
            if nums[i] > nums[i + step] {
                flag = true;
                nums.swap(i, i + step);
            }
            i += step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors_basic() {
        let mut vec_1 = vec![2, 0, 2, 1, 1, 0];
        let mut vec_2 = vec![2, 0, 1];
        sort_colors(&mut vec_1);
        sort_colors(&mut vec_2);
        assert!(vec_1.is_sorted());
        assert!(vec_2.is_sorted());
    }
}
