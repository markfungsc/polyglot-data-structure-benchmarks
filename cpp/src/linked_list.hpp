#pragma once
#include <cstddef>

namespace linked_list {
struct LinkedList {
    void push(int x);
    int get(size_t i) const;
    size_t length() const;
};
}
