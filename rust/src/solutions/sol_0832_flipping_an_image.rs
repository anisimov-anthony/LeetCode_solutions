#![allow(dead_code)]
pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::with_capacity(image.len());
    for row in image.iter() {
        let mut new_row = Vec::with_capacity(image[0].len());
        for i in row.iter().rev() {
            let new_i = match i {
                1 => 0,
                _ => 1,
            };
            new_row.push(new_i);
        }
        result.push(new_row);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_and_invert_image_basic() {
        assert_eq!(
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );
        assert_eq!(
            flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
