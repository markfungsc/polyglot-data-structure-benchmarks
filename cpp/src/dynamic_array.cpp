#include "dynamic_array.hpp"
#include <algorithm> // for std::move
#include <cstdint>

namespace dynamic_array {

// Constructor: start empty with small initial capacity (e.g., 4)
template <typename T>
DynamicArray<T>::DynamicArray() : data_(nullptr), capacity_(4), size_(0) {
    data_ = alloc_.allocate(capacity_);
}

// Destructor: free the allocated memory
template <typename T>
DynamicArray<T>::~DynamicArray() {
    for (size_t i = 0; i < size_; ++i) {
        alloc_.destroy(&data_[i]);
    }
    alloc_.deallocate(data_, capacity_);
}

// Append a value to the end of the array
template <typename T>
inline void DynamicArray<T>::push(const T& value) {
    if (size_ == capacity_) {
        resize();
    }
    alloc_.construct(&data_[size_], value);
    ++size_;
}

// Get the value at the given index
template <typename T>
inline T& DynamicArray<T>::get(size_t index) const {
    if (index >= size_) {
        throw std::out_of_range("DynamicArray: Index out of range");
    }
    return const_cast<T&>(data_[index]);
}

// Get the number of elements in the array
template <typename T>
size_t DynamicArray<T>::size() const { return size_; }

// Get the allocated capacity of the array
template <typename T>
size_t DynamicArray<T>::capacity() const { return capacity_; }

// Resize the array to the new capacity
template <typename T>
void DynamicArray<T>::resize() {
    size_t new_capacity = capacity_ * 2;
    T* new_data = alloc_.allocate(new_capacity);

    // Move or copy each element to the new array
    for (size_t i = 0; i < size_; ++i) {
        alloc_.construct(&new_data[i], std::move(data_[i]));
        alloc_.destroy(&data_[i]);
    }
    alloc_.deallocate(data_, capacity_);
    data_ = new_data;
    capacity_ = new_capacity;
}

template <typename T>
inline void do_not_optimize(const T& value) {
    asm volatile("" : : "g"(value) : "memory");
}

}

// Explicit template instantiation for int (since benchmark uses int)
template class dynamic_array::DynamicArray<std::int32_t>;