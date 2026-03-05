#pragma once
#include <cstddef>
#include <memory>

namespace heap {

template <typename T, typename Alloc = std::allocator<T>>
struct MinHeap {
   public:
    // Constructor
    explicit MinHeap(size_t capacity = 16, const Alloc& alloc = Alloc());
    // Destructor
    ~MinHeap();

    // Disable copy constructor and copy assignment
    MinHeap(const MinHeap&) = delete;
    MinHeap& operator=(const MinHeap&) = delete;

    // Move constructor and move assignment
    MinHeap(MinHeap&& other) noexcept;
    MinHeap& operator=(MinHeap&& other) noexcept;

    void insert(const T& value);
    void insert(T&& value);
    T pop();
    const T& peek() const;
    size_t size() const noexcept;
    bool empty() const noexcept;

   private:
    using traits = std::allocator_traits<Alloc>;
    void siftUp(size_t index);
    void siftDown(size_t index);
    void resize();
    void clear();

    T* data_;
    size_t size_;
    size_t capacity_;
    Alloc alloc_;
};
}  // namespace heap
