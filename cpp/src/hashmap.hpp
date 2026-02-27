#pragma once
#include <cstddef>

namespace hashmap {
struct HashMap {
    int get(int key) const;
    void put(int key, int value);
    void remove(int key);
    size_t size() const;
};
}
