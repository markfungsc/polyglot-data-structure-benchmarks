#pragma once

namespace lru_cache {
struct LRUCache {
    explicit LRUCache(size_t /*capacity*/) {}
    int get(int key);
    void put(int key, int value);
};
}
