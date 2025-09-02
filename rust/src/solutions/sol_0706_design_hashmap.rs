#![allow(dead_code)]
struct MyHashMap {
    store: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            store: vec![-1; 1000000 + 1],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.store[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.store[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.store[key as usize] = -1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_hashmap_basic() {
        let mut obj = MyHashMap::new();
        obj.put(1, 3);
        assert_eq!(obj.get(1), 3);
        obj.remove(1);
        assert_eq!(obj.get(1), -1);
    }
}
