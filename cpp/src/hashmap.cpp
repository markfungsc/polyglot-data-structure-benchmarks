#include "hashmap.hpp"

namespace hashmap {

HashMap::HashMap(size_t initial_capacity) : capacity_(initial_capacity), size_(0) {
    buckets_.resize(capacity_);
    for (auto& bucket : buckets_) {
        bucket.reserve(2);  // small preallocation
    }
}

size_t HashMap::hash(int key) const {
    size_t h = static_cast<size_t>(key);
    return (h & 0x7FFF'FFFFu) % capacity_;
}

int HashMap::get(int key) const {
    size_t idx = hash(key);
    for (const auto& p : buckets_[idx]) {
        if (p.first == key)
            return p.second;
    }
    return 0;
}

void HashMap::insert(int key, int value) {
    size_t idx = hash(key);
    for (auto& p : buckets_[idx]) {
        if (p.first == key) {
            p.second = value;
            return;
        }
    }
    buckets_[idx].emplace_back(key, value);
    ++size_;
    if (size_ > static_cast<size_t>(LOAD_FACTOR * capacity_))
        resize();
}

void HashMap::remove(int key) {
    size_t idx = hash(key);
    auto& bucket = buckets_[idx];
    for (auto it = bucket.begin(); it != bucket.end(); ++it) {
        if (it->first == key) {
            bucket.erase(it);
            --size_;
            return;
        }
    }
}

size_t HashMap::size() const {
    return size_;
}

void HashMap::resize() {
    size_t new_cap = capacity_ * 2;
    std::vector<std::vector<std::pair<int, int>>> new_buckets(new_cap);
    for (auto& bucket : buckets_) {
        for (auto& p : bucket) {
            size_t h = static_cast<size_t>(p.first);
            size_t idx = (h & 0x7FFF'FFFFu) % new_cap;
            new_buckets[idx].emplace_back(std::move(p));
        }
    }
    buckets_ = std::move(new_buckets);
    capacity_ = new_cap;
}

}  // namespace hashmap
