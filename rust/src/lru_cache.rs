pub struct LRUCache {
    _capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache { _capacity: capacity }
    }
    pub fn get(&mut self, _key: i32) -> Option<i32> {
        None
    }
    pub fn put(&mut self, _key: i32, _value: i32) {}
}
