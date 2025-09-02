#![allow(dead_code)]
struct MyHashSet {
    store: Vec<i32>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            store: vec![-1; 1000000 + 1],
        }
    }

    fn add(&mut self, key: i32) {
        self.store[key as usize] = 1;
    }

    fn remove(&mut self, key: i32) {
        self.store[key as usize] = -1;
    }

    fn contains(&self, key: i32) -> bool {
        match self.store[key as usize] {
            1 => return true,
            -1 => return false,
            _ => panic!("oh no!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_hashset_basic() {
        let mut obj = MyHashSet::new();
        obj.add(1);
        assert!(obj.contains(1));
        obj.remove(1);
        assert!(!obj.contains(1));
    }
}
