package com.polyglot;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class LRUCacheTest {
  @Test
  void newAcceptsCapacity() {
    LRUCache cache = new LRUCache(4);
    assertNotNull(cache);
  }

  @Test
  void getReturnsNullStub() {
    LRUCache cache = new LRUCache(2);
    assertNull(cache.get(1));
  }

  @Test
  void putDoesNotThrowStub() {
    LRUCache cache = new LRUCache(2);
    assertDoesNotThrow(() -> cache.put(1, 10));
  }
}
