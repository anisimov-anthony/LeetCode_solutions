#[allow(dead_code)]
struct MinStack {
    storage: Vec<i32>,
    min_stack: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self {
            storage: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.storage.push(val);
        let min_val = if self.min_stack.is_empty() {
            val
        } else {
            val.min(*self.min_stack.last().unwrap())
        };
        self.min_stack.push(min_val);
    }

    fn pop(&mut self) {
        if !self.storage.is_empty() {
            self.storage.pop();
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.storage.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut minstack = MinStack::new();
        minstack.push(-2);
        minstack.push(0);
        minstack.push(-3);

        assert_eq!(minstack.get_min(), -3);

        minstack.pop();

        assert_eq!(minstack.top(), 0);
        assert_eq!(minstack.get_min(), -2);
    }
}
