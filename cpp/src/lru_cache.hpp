#pragma once
#include <cstddef>
#include <unordered_map>

namespace lru_cache {
class LRUCache {
   public:
    explicit LRUCache(size_t capacity);
    ~LRUCache();
    int get(int key);
    void put(int key, int value);
    size_t size() const;
    size_t capacity() const;

   private:
    struct Node {
        int key;
        int value;
        Node* prev;
        Node* next;
    };

    void move_to_front(Node* node);
    void remove_node(Node* node);
    void add_to_front(Node* node);
    Node* remove_lru();

    size_t capacity_;
    size_t size_;
    // uses unordered_map (standard library)
    std::unordered_map<int, Node*> map_;  // key -> node pointer
    Node* head_;                          // MRU (most recently used) Node
    Node* tail_;                          // LRU (least recently used) Node
};
}  // namespace lru_cache
