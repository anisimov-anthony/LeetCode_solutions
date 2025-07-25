#![allow(dead_code)]
pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let mut result = 0;
    for i in 0..start_time.len() {
        if start_time[i] <= query_time && end_time[i] >= query_time {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busy_student_basic() {
        assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(busy_student(vec![4], vec![4], 4), 1);
    }
}
