#[allow(dead_code)]
struct CustomStack {
    store: Vec<i32>,
    max_size: usize,
}

#[allow(dead_code)]
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            store: Vec::with_capacity(max_size as usize),
            max_size: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.store.len() < self.max_size {
            self.store.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.store.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = std::cmp::min(k as usize, self.store.len());
        for i in 0..k {
            self.store[i] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_stack_basic() {
        let mut obj = CustomStack::new(10);
        obj.push(10);
        obj.push(11);
        let ret_2: i32 = obj.pop();
        assert_eq!(ret_2, 11);
        obj.increment(1, 10);
        let ret_3: i32 = obj.pop();
        assert_eq!(ret_3, 20);
    }
}
