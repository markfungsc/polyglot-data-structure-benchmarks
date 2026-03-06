package com.polyglot;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class LRUCacheTest {
  @Test
  void newAcceptsCapacity() {
    LRUCache cache = new LRUCache(4);
    assertEquals(4, cache.capacity());
    assertEquals(0, cache.size());
  }

  @Test
  void get() {
    LRUCache cache = new LRUCache(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assertEquals(10, cache.get(1));
    assertEquals(20, cache.get(2));
    assertNull(cache.get(3));
  }

  @Test
  void put() {
    LRUCache cache = new LRUCache(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assertEquals(10, cache.get(1));
    assertEquals(20, cache.get(2));
    assertNull(cache.get(3));
    cache.put(3, 30);
    assertEquals(20, cache.get(2));
    assertEquals(30, cache.get(3));
    assertNull(cache.get(1));
  }
}
