// src/hashmap.rs
use ahash::AHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;
use std::hash::{BuildHasher, Hash, Hasher};

type ABuildHasher = BuildHasherDefault<AHasher>;

pub struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct HashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>, // separate chaining
    capacity: usize,
    size: usize,
    build_hasher: ABuildHasher,
}

impl<K: Eq + Hash + Clone, V: Clone> HashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        let buckets = (0..capacity).map(|_| Vec::new()).collect();
        Self {
            buckets,
            capacity,
            size: 0,
            build_hasher: ABuildHasher::default(),
        }
    }

    fn hash(&self, key: &K) -> usize {
        (self.build_hasher.hash_one(key) as usize) % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) {
        let idx = self.hash(&key);
        for entry in &mut self.buckets[idx] {
            if entry.key == key {
                entry.value = value;
                return;
            }
        }
        self.buckets[idx].push(Entry { key, value });
        self.size += 1;
        if self.size > self.capacity * 3 / 4 {
            self.resize();
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.hash(key);
        for entry in &self.buckets[idx] {
            if &entry.key == key {
                return Some(&entry.value);
            }
        }
        None
    }

    fn resize(&mut self) {
        self.capacity *= 2;
        let mut new_buckets: Vec<Vec<Entry<K, V>>> =
            (0..self.capacity).map(|_| Vec::new()).collect();
        for bucket in &self.buckets {
            for entry in bucket {
                let idx = {
                    let mut hasher = DefaultHasher::new();
                    entry.key.hash(&mut hasher);
                    (hasher.finish() as usize) % self.capacity
                };
                new_buckets[idx].push(Entry {
                    key: entry.key.clone(),
                    value: entry.value.clone(),
                });
            }
        }
        self.buckets = new_buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let m: HashMap<i32, i32> = HashMap::new(8);
        assert_eq!(m.get(&1), None);
    }

    #[test]
    fn insert_and_get() {
        let mut m = HashMap::new(8);
        m.insert(1, 10);
        m.insert(2, 20);
        assert_eq!(m.get(&1), Some(&10));
        assert_eq!(m.get(&2), Some(&20));
        assert_eq!(m.get(&3), None);
    }

    #[test]
    fn insert_overwrites() {
        let mut m = HashMap::new(8);
        m.insert(1, 10);
        m.insert(1, 20);
        assert_eq!(m.get(&1), Some(&20));
    }
}
