class Entry:
    def __init__(self, key, value):
        self.key = key
        self.value = value


class HashMap:
    def __init__(self, capacity=16):
        self.capacity = capacity
        self.size = 0
        self.buckets = [[] for _ in range(capacity)]  # list of lists

    def _hash(self, key):
        return hash(key) % self.capacity

    def insert(self, key, value):
        idx = self._hash(key)
        bucket = self.buckets[idx]

        # update if key exists
        for entry in bucket:
            if entry.key == key:
                entry.value = value
                return

        # insert new
        bucket.append(Entry(key, value))
        self.size += 1

        # resize if load factor > 0.75
        if self.size > 0.75 * self.capacity:
            self._resize()

    def get(self, key):
        idx = self._hash(key)
        bucket = self.buckets[idx]
        for entry in bucket:
            if entry.key == key:
                return entry.value
        return None

    def _resize(self):
        old_buckets = self.buckets
        self.capacity *= 2
        self.buckets = [[] for _ in range(self.capacity)]
        self.size = 0

        for bucket in old_buckets:
            for entry in bucket:
                self.insert(entry.key, entry.value)