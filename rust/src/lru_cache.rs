use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

// Use Rc<RefCell<>> here for studying data structure, borrowing, ownership, etc.
// not optimising for performance because it adds runtime overhead, Vec + index pointer is faster for this use case.
type NodePtr<K, V> = Rc<RefCell<Node<K, V>>>;

pub struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<NodePtr<K, V>>,
    next: Option<NodePtr<K, V>>,
}

// Double linked list with a hash map for O(1) access
// - The head of the list is the least recently used node (LRU)
// - The tail of the list is the most recently used node (MRU)
// - The hash map is used to map keys to nodes
// - The nodes are stored in the hash map and the list
pub struct LRUCache<K, V> {
    map: HashMap<K, NodePtr<K, V>>, // key -> node pointer
    head: Option<NodePtr<K, V>>,    // LRU (least recently used) Node
    tail: Option<NodePtr<K, V>>,    // MRU (most recently used) Node
    capacity: usize,                // maximum number of nodes in the cache
}

impl<K: Eq + Hash + Clone, V: Clone> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    // Get the value for a key, and move the node to the back of the list (MRU)
    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.map.get(&key).cloned() {
            let value = node.borrow().value.clone();
            self.move_to_tail(node);
            Some(value)
        } else {
            None
        }
    }

    // Put a key-value pair into the cache
    // If the key is already in the cache, update the value
    // If the cache is full, evict the least recently used node (LRU)
    // If the key is not in the cache, add it to the cache
    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.move_to_tail(node);
            return;
        }

        if self.map.len() == self.capacity {
            self.evict_lru();
        }

        let node = Node {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        };
        let node_ptr = Rc::new(RefCell::new(node));
        self.push_back(node_ptr.clone());
        self.map.insert(key, node_ptr);
    }

    /// Push a node to the back of the list (MRU)
    fn push_back(&mut self, node: NodePtr<K, V>) {
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    // Remove a node from the list and update the head and tail pointers (if necessary)
    fn remove(&mut self, node: NodePtr<K, V>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();
        match prev {
            Some(ref p) => {
                p.borrow_mut().next = next.clone();
            }
            None => {
                self.head = next.clone();
            }
        }

        match next {
            Some(ref n) => {
                n.borrow_mut().prev = prev.clone();
            }
            None => {
                self.tail = prev.clone();
            }
        }
    }

    // Move a node to the back of the list (MRU)
    fn move_to_tail(&mut self, node: NodePtr<K, V>) {
        self.remove(node.clone());
        self.push_back(node);
    }

    // Evict the least recently used node (LRU)
    fn evict_lru(&mut self) {
        if let Some(node) = self.head.take() {
            let key = node.borrow().key.clone();
            self.remove(node);
            self.map.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_capacity() {
        let cache = LRUCache::<i32, i32>::new(4);
        assert_eq!(cache.capacity, 4);
        assert_eq!(cache.map.len(), 0);
        assert!(cache.head.is_none());
        assert!(cache.tail.is_none());
    }

    #[test]
    fn get() {
        let mut cache = LRUCache::<i32, i32>::new(2);
        assert_eq!(cache.get(1), None);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), None);
    }

    #[test]
    fn put() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), None);
        cache.put(3, 30);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }
}
