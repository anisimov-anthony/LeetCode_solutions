#![allow(dead_code)]
pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    mountain
        .windows(3)
        .enumerate()
        .filter(|(_, mn)| (mn[0] < mn[1]) && (mn[1] > mn[2]))
        .map(|(i, _)| (i + 1) as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peaks_basic() {
        assert_eq!(find_peaks(vec![2, 4, 4]), Vec::<i32>::new());
        assert_eq!(find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }
}
