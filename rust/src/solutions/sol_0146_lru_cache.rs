use ::std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
struct LRUCache {
    data: HashMap<i32, i32>,
    history: VecDeque<i32>,
}

#[allow(dead_code)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            data: HashMap::new(),
            history: VecDeque::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.data.get(&key) {
            self.history.retain(|&k| k != key);
            self.history.push_front(key);
            return *value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.data.contains_key(&key) {
            self.data.insert(key, value);
            self.history.retain(|&k| k != key);
        } else {
            if self.history.len() == self.history.capacity() {
                if let Some(oldest) = self.history.pop_back() {
                    self.data.remove(&oldest);
                }
            }
            self.data.insert(key, value);
        }
        self.history.push_front(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
