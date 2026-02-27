#pragma once
#include <cstddef>

namespace dynamic_array {
struct DynamicArray {
    void push(int x);
    int get(size_t i) const;
    size_t length() const;
};
}
