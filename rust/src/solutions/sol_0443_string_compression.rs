#![allow(dead_code)]
pub fn compress(chars: &mut Vec<char>) -> i32 {
    if chars.is_empty() {
        return 0;
    }

    let initial_size = chars.len();
    let mut drain_size = 0;

    let mut last = chars[0];
    let mut freq = 0;

    for i in 0..initial_size {
        if chars[i] == last {
            freq += 1;
        } else {
            chars.push(last);
            if freq != 1 {
                let mut length: Vec<_> = freq.to_string().chars().collect();
                chars.append(&mut length);
            }
            last = chars[i];
            drain_size += freq;
            freq = 1;
        }
    }
    chars.push(last);
    if freq != 1 {
        let mut length: Vec<_> = freq.to_string().chars().collect();
        chars.append(&mut length);
    }
    drain_size += freq;

    chars.drain(0..drain_size);

    chars.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_basic() {
        assert_eq!(compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']), 6);
        assert_eq!(compress(&mut vec!['a']), 1);
        assert_eq!(
            compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ]),
            4
        );
    }
}
