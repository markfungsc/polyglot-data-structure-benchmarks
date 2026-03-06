package com.polyglot;

import java.util.LinkedHashMap;
import java.util.Map;

// Using LinkedHashMap instead of manual implementation to study real-world LRU cache implementation
// in Java.
public class LRUCache {
  private final int capacity;
  private final LinkedHashMap<Integer, Integer> map;

  public LRUCache(int capacity) {
    this.capacity = capacity;
    this.map =
        new LinkedHashMap<Integer, Integer>(capacity, 0.75f, true) {
          // Override the removeEldestEntry method to remove the least recently used entry when the
          // cache is full
          @Override
          protected boolean removeEldestEntry(Map.Entry<Integer, Integer> eldest) {
            return size() > LRUCache.this.capacity;
          }
        };
  }

  public Integer get(Integer key) {
    return map.getOrDefault(key, null);
  }

  public void put(Integer key, Integer value) {
    map.put(key, value);
  }

  public int size() {
    return map.size();
  }

  public int capacity() {
    return capacity;
  }
}
