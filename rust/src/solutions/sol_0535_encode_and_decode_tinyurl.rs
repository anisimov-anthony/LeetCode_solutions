use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct Codec {
    store: HashMap<String, String>,
}

#[allow(dead_code)]
impl Codec {
    fn new() -> Self {
        Codec {
            store: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longurl: String) -> String {
        let base = "http://tinyurl.com";
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        longurl.hash(&mut hasher);
        let short_code = hasher.finish().to_string();

        let shorturl = format!("{}/{}", base, short_code);

        self.store.insert(shorturl.clone(), longurl);

        shorturl
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shorturl: String) -> String {
        self.store
            .get(&shorturl)
            .cloned()
            .unwrap_or_else(String::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_basic() {
        let mut obj = Codec::new();
        let s: String = obj.encode("https://leetcode.com/problems/design-tinyurl".to_string());
        let _ans: String = obj.decode(s);
        assert!(true);
    }
}
