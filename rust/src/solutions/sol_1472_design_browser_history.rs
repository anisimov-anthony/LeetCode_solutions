#![allow(dead_code)]
struct BrowserHistory {
    pages: Vec<String>,
    current: usize,
}

#[allow(dead_code)]
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            pages: vec![homepage],
            current: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.current += 1;
        self.pages.truncate(self.current);
        self.pages.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        self.current = self.current.saturating_sub(steps as usize);
        self.pages[self.current].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current = (self.pages.len() - 1).min(self.current + steps as usize);
        self.pages[self.current].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_history_basic() {
        let mut obj = BrowserHistory::new("leetcode.com".to_string());
        obj.visit("youtube.com".to_string());
        let ret_2: String = obj.back(1);
        let ret_3: String = obj.forward(1);

        assert_eq!(ret_2, "leetcode.com".to_string());
        assert_eq!(ret_3, "youtube.com".to_string());
    }
}
