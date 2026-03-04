import heapq

# Uses heapq library (implemented in C) to implement a min heap
# for much faster performance than the custom implementation
class MinHeap:
    def __init__(self):
        self.data = []

    def insert(self, value: int):
        heapq.heappush(self.data, value) # O(log n)

    def pop(self):
        if not self.data:
            return None
        return heapq.heappop(self.data) # O(log n)

    def peek(self):
        if len(self.data) == 0:
            return None
        return self.data[0] # O(1)

    def size(self):
        return len(self.data)
