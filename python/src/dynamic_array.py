import numpy as np


class DynamicArray:
    def __init__(self, capacity: int, dtype=np.int32):
        """
        capacity: initial allocated size
        dtype: type of array elements (np.int32, np.int64, etc.)
        """
        self.capacity = capacity
        self.size = 0
        self.data = np.empty(self.capacity, dtype=dtype)

    def push(self, value: int):
        if self.size >= self.capacity:
            self.resize()
        self.data[self.size] = value
        self.size += 1

    def get(self, index: int) -> int:
        if index < 0 or index >= self.size:
            raise IndexError(f"DynamicArray: Index {index} out of bounds")
        return self.data[index]

    def resize(self):
        new_capacity = self.capacity * 2
        new_data = np.empty(new_capacity, dtype=self.data.dtype)
        new_data[: self.size] = self.data[: self.size]  # copy old elements
        self.data = new_data
        self.capacity = new_capacity

    def __len__(self):
        return self.size
