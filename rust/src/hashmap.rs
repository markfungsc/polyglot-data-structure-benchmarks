// src/hashmap.rs
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, Hash, Hasher};
use ahash::AHasher;
use std::hash::BuildHasherDefault;

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
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
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
        let mut new_buckets: Vec<Vec<Entry<K, V>>> = (0..self.capacity).map(|_| Vec::new()).collect();
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