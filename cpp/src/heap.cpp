#include "heap.hpp"
#include <cstddef>
#include <stdexcept>

namespace heap {

template<typename T, typename Alloc>
MinHeap<T, Alloc>::MinHeap(size_t capacity, const Alloc& alloc)
    : size_(0), capacity_(capacity ? capacity : 1), alloc_(alloc)
{
    // allocate the memory for the heap
    data_ = traits::allocate(alloc_, capacity_);
}

template<typename T, typename Alloc>
MinHeap<T, Alloc>::~MinHeap() {
    // destroy the heap
    for (size_t i = 0; i < size_; ++i) {
        alloc_.destroy(&data_[i]);
    }
    alloc_.deallocate(data_, capacity_);
}

template<typename T, typename Alloc>
MinHeap<T, Alloc>::MinHeap(MinHeap&& other) noexcept
    : data_(other.data_),
      size_(other.size_),
      capacity_(other.capacity_),
      alloc_(std::move(other.alloc_))
{
    other.data_ = nullptr;
    other.size_ = 0;
    other.capacity_ = 0;
}

template<typename T, typename Alloc>
MinHeap<T, Alloc>&
MinHeap<T, Alloc>::operator=(MinHeap&& other) noexcept {
    if (this != &other) {
        clear();
        // deallocate the memory for the heap
        traits::deallocate(alloc_, data_, capacity_);

        // move the data from the other heap
        data_ = other.data_;
        size_ = other.size_;
        capacity_ = other.capacity_;
        alloc_ = std::move(other.alloc_);

        other.data_ = nullptr;
        other.size_ = 0;
        other.capacity_ = 0;
    }
    return *this;
}

template<typename T, typename Alloc>
size_t MinHeap<T, Alloc>::size() const noexcept {
    return size_;
}

template<typename T, typename Alloc>
bool MinHeap<T, Alloc>::empty() const noexcept {
    return size_ == 0;
}

template<typename T, typename Alloc>
const T& MinHeap<T, Alloc>::peek() const {
    if (empty())
        throw std::out_of_range("Heap is empty");
    return data_[0];
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::insert(const T& value) {
    if (size_ == capacity_)
        resize();

    // construct the new element
    traits::construct(alloc_, data_ + size_, value);
    // sift up the new element
    siftUp(size_);
    // increment the size
    ++size_;
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::insert(T&& value) {
    if (size_ == capacity_)
        resize();

    // construct the new element
    traits::construct(alloc_, data_ + size_, std::move(value));
    // sift up the new element
    siftUp(size_);
    // increment the size
    ++size_;
}

template<typename T, typename Alloc>
T MinHeap<T, Alloc>::pop() {
    if (empty())
        throw std::out_of_range("Heap is empty");

    // get the minimum value
    T result = std::move(data_[0]);

    // decrement the size
    --size_;

    if (size_ > 0) {
        // destroy the root element
        traits::destroy(alloc_, data_);
        // construct the new root element (the last element)
        traits::construct(alloc_, data_, std::move(data_[size_]));
        // destroy the last element
        traits::destroy(alloc_, data_ + size_);
        // sift down the new root element
        siftDown(0);
    } else {
        // destroy the root element (leaves the heap empty)
        traits::destroy(alloc_, data_);
    }

    // return the minimum value
    return result;
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::clear() {
    // destroy the heap
    for (size_t i = 0; i < size_; ++i)
        traits::destroy(alloc_, data_ + i);
    // set the size to 0
    size_ = 0;
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::siftUp(size_t index) {
    // get the value at the index
    T value = std::move(data_[index]);

    while (index > 0) {
        // get the parent index
        size_t parent = (index - 1) / 2;

        // if the value is greater than the parent, break
        if (!(value < data_[parent]))
            break;

        // move the parent down
        data_[index] = std::move(data_[parent]);
        // update the index to the parent
        index = parent;
    }

    // update the value at the correct index
    data_[index] = std::move(value);
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::siftDown(size_t index) {
    // get the value at the index
    T value = std::move(data_[index]);

    while (true) {
        // get the left child index
        size_t left = 2 * index + 1;
        // if the left child is out of bounds, break
        if (left >= size_)
            break;

        // get the right child index
        size_t right = left + 1;
        // get the smallest index
        size_t smallest = left;

        // if the right child is in bounds and is smaller than the left child, update the smallest index
        if (right < size_ && data_[right] < data_[left])
            smallest = right;

        // if the smallest value is greater than the value, break
        if (!(data_[smallest] < value))
            break;

        // move the smallest value up
        data_[index] = std::move(data_[smallest]);
        index = smallest;
    }

    // update the value at the correct index
    data_[index] = std::move(value);
}

template<typename T, typename Alloc>
void MinHeap<T, Alloc>::resize() {
    // get the new capacity
    size_t new_capacity = capacity_ * 2;
    // allocate the memory for the new data
    T* new_data = traits::allocate(alloc_, new_capacity);

    // move the data to the new data
    for (size_t i = 0; i < size_; ++i) {
        // construct the new element
        traits::construct(alloc_, new_data + i, std::move(data_[i]));
        traits::destroy(alloc_, data_ + i);
    }

    traits::deallocate(alloc_, data_, capacity_);

    data_ = new_data;
    capacity_ = new_capacity;
}

}

template class heap::MinHeap<int>;