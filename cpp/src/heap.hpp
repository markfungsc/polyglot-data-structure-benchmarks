#pragma once
#include <cstddef>

namespace heap {
struct Heap {
    void push(int x);
    int peek() const;
    int pop();
    size_t size() const;
};
}
