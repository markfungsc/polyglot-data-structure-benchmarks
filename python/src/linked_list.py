from typing import Callable


class LinkedList:
    class _Node:
        # Slots are used to reduce memory usage by avoiding the creation of a dictionary for each instance
        __slots__ = ("value", "next")

        def __init__(self, value):
            self.value = value
            self.next = None

    def __init__(self):
        self.head = None  # head node
        self.tail = None  # tail node (for fast append)
        self._size = 0

    def push_back(self, value: int):
        new_node = self._Node(value)

        if self.tail is None:
            # Empty list, head and tail both point to new node
            self.head = new_node
            self.tail = new_node
        else:
            # List is not empty, add new node after tail and update tail
            self.tail.next = new_node
            self.tail = new_node

        self._size += 1

    def get(self, index: int) -> int:
        if index < 0 or index >= self._size:
            return None
        current = self.head
        # Traverse the list until we find the node at the given index
        for _ in range(index):
            current = current.next
        return current.value if current else None

    def delete(self, index: int) -> bool:
        if index < 0 or index >= self._size:
            return False
        # Remove the first node if the index is 0
        if index == 0:
            self.head = self.head.next
            if self._size == 1:  # list becomes empty, reset tail
                self.tail = None
            self._size -= 1
            return True
        # Traverse the list until we find the node before the one at the given index
        current = self.head
        for _ in range(index - 1):
            current = current.next if current else None
        # Remove the node at the given index
        if current and current.next:
            if current.next == self.tail:  # deleting last node, update tail
                self.tail = current
            current.next = current.next.next
            self._size -= 1
            return True
        return False

    def traverse(self, f: Callable[[int], None]):
        current = self.head
        while current:
            f(current.value)
            current = current.next

    def size(self) -> int:
        return self._size
