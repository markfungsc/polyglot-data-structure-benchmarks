#pragma once
#include <cstddef>  // for size_t
#include <memory>   // for std::allocator

namespace dynamic_array {

template <typename T>
class DynamicArray {
   public:
    DynamicArray();   // Constructor
    ~DynamicArray();  // Destructor

    void push(const T& value);   // Append
    T& get(size_t index) const;  // Get by index
    size_t size() const;         // Number of elements
    size_t capacity() const;     // Allocated capacity

   private:
    void resize();

    T* data_;                  // Pointer to the allocated memory
    size_t size_;              // Number of elements
    size_t capacity_;          // Allocated capacity
    std::allocator<T> alloc_;  // Allocator
};
}  // namespace dynamic_array
