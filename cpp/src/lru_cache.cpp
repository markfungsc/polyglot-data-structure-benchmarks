#include "lru_cache.hpp"

namespace lru_cache {

LRUCache::LRUCache(size_t capacity)
    : capacity_(capacity), size_(0), head_(nullptr), tail_(nullptr) {
    map_.reserve(capacity);
}

LRUCache::~LRUCache() {
    Node* current = head_;
    while (current) {
        Node* next = current->next;
        delete current;
        current = next;
    }
}

int LRUCache::get(int key) {
    auto it = map_.find(key);
    if (it == map_.end()) {
        return -1;
    }
    Node* node = it->second;
    move_to_front(node);
    return node->value;
}

void LRUCache::put(int key, int value) {
    auto it = map_.find(key);
    if (it != map_.end()) {
        Node* node = it->second;
        node->value = value;
        move_to_front(node);
        return;
    }

    Node* node = new Node{key, value, nullptr, nullptr};

    map_[key] = node;
    add_to_front(node);
    size_++;
    if (size_ > capacity_) {
        // remove the last node from the list
        Node* lru = remove_lru();
        // remove the least recently used node from the map
        map_.erase(lru->key);
        // delete the node
        delete lru;
        // decrement the size
        size_--;
    }
}

void LRUCache::move_to_front(Node* node) {
    remove_node(node);
    add_to_front(node);
}

// remove the node from the list
void LRUCache::remove_node(Node* node) {
    if (node->prev)
        node->prev->next = node->next;
    else
        head_ = node->next;

    if (node->next)
        node->next->prev = node->prev;
    else
        tail_ = node->prev;
}

// add the node to the front of the list
void LRUCache::add_to_front(Node* node) {
    node->next = head_;
    node->prev = nullptr;

    if (head_) head_->prev = node;

    head_ = node;

    if (!tail_) tail_ = node;
}

LRUCache::Node* LRUCache::remove_lru() {
    Node* node = tail_;
    remove_node(node);
    return node;
}

size_t LRUCache::size() const { return size_; }

size_t LRUCache::capacity() const { return capacity_; }

}  // namespace lru_cache