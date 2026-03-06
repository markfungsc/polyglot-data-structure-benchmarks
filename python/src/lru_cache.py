from collections import OrderedDict


# Using OrderedDict (C implementation) instead of manual implementation to optimise performance.
# There is a faster functools.lru_cache in Python that is a LRU cache implementation but no explicit get()/ put() access to the cache.
class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key: int):  # O(1)
        if key not in self.cache:
            return None

        # move the key to the end of the OrderedDict (MRU)
        self.cache.move_to_end(key)
        return self.cache[key]

    def put(self, key: int, value: int):  # O(1)
        if key in self.cache:
            # update the value
            self.cache[key] = value
            # move the key to the end of the OrderedDict (MRU)
            self.cache.move_to_end(key)
            return

        # if the cache is full, evict the least recently used key (LRU)
        if len(self.cache) >= self.capacity:
            # evict the least recently used key (LRU)
            self.cache.popitem(last=False)

        # insert the new key-value pair
        self.cache[key] = value
