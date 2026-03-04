#include "linked_list.hpp"
#include <stdexcept>

namespace linked_list {

LinkedList::LinkedList() : head_(nullptr), tail_(nullptr), size_(0) {}

LinkedList::~LinkedList() {
    Node* current = head_;
    while (current != nullptr) {
        Node* next = current->next;
        delete current;
        current = next;
    }
}

void LinkedList::pushBack(int value) {
    Node* newNode = new Node{value, nullptr};
    if (head_ == nullptr) {
        head_ = newNode;
        tail_ = newNode;
    } else {
        tail_->next = newNode;
        tail_ = newNode;
    }
    size_++;
}

int LinkedList::get(size_t index) const {
    if (index >= size_) {
        throw std::out_of_range("LinkedList: Index out of range");
    }
    Node* current = head_;
    for (size_t i = 0; i < index; i++) {
        current = current->next;
    }
    return current->value;
}

void LinkedList::deleteNode(size_t index) {
    if (index >= size_) {
        throw std::out_of_range("LinkedList: Index out of range");
    }
    // Remove the first node if the index is 0
    if (index == 0) {
        Node* old_head = head_;
        head_ = head_->next;
        if (head_ == nullptr) tail_ = nullptr;
        delete old_head;
        size_--;
    } else {
        Node* current = head_;
        for (size_t i = 0; i < index - 1; i++) current = current->next;
        Node* old_node = current->next;
        current->next = current->next->next;
        if (current->next == nullptr) tail_ = current;
        delete old_node;
        size_--;
    }
}

size_t LinkedList::size() const {
    return size_;
}

void LinkedList::traverse(std::function<void(int)> const& f) {
    Node* current = head_;
    while (current != nullptr) {
        f(current->value);
        current = current->next;
    }
}

}
