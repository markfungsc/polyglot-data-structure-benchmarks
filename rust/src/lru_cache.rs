pub struct LRUCache {
    _capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            _capacity: capacity,
        }
    }
    pub fn get(&mut self, _key: i32) -> Option<i32> {
        None
    }
    pub fn put(&mut self, _key: i32, _value: i32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_capacity() {
        let _cache = LRUCache::new(4);
    }

    #[test]
    fn get_returns_none_stub() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(1), None);
    }

    #[test]
    fn put_does_not_panic_stub() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
    }
}
