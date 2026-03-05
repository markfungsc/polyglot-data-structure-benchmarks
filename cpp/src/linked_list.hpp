#pragma once
#include <cstddef>
#include <functional>

namespace linked_list {
class LinkedList {
   public:
    LinkedList();
    ~LinkedList();
    void pushBack(int value);
    int get(size_t index) const;
    void deleteNode(size_t index);
    size_t size() const;
    void traverse(std::function<void(int)> const& f);

    // disable copy constructor and copy assignment
    LinkedList(const LinkedList&) = delete;
    LinkedList& operator=(const LinkedList&) = delete;

   private:
    struct Node {
        int value;
        Node* next;
    };
    Node* head_;
    Node* tail_;
    size_t size_;
};
}  // namespace linked_list
