#pragma once
#include <cstddef>
#include <utility>
#include <vector>

namespace hashmap {
    struct HashMap {
        HashMap(size_t initial_capacity);

        int get(int key) const;
        void insert(int key, int value);
        void remove(int key);
        size_t size() const;

    private:
        static constexpr double LOAD_FACTOR = 0.75;

        std::vector<std::vector<std::pair<int, int>>> buckets_;
        size_t capacity_;
        size_t size_;

        size_t hash(int key) const;
        void resize();
    };
}
