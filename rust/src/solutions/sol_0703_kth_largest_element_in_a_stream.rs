#![allow(dead_code)]
struct KthLargest {
    stream: Vec<i32>,
    kth: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            stream: nums,
            kth: k,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.stream.push(val);
        self.stream.sort_unstable();
        self.stream[(self.stream.len() - self.kth as usize).max(0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest_basic() {
        let mut kl = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kl.add(3), 4);
        assert_eq!(kl.add(5), 5);
        assert_eq!(kl.add(10), 5);
        assert_eq!(kl.add(9), 8);
        assert_eq!(kl.add(4), 8);

        let mut kl = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kl.add(2), 7);
        assert_eq!(kl.add(10), 7);
        assert_eq!(kl.add(9), 7);
        assert_eq!(kl.add(9), 8);
    }
}
