package com.polyglot;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class HashMapCustomTest {
  @Test
  void newIsEmpty() {
    HashMapCustom<Integer, Integer> m = new HashMapCustom<>(8);
    assertNull(m.get(1));
  }

  @Test
  void insertAndGet() {
    HashMapCustom<Integer, Integer> m = new HashMapCustom<>(8);
    m.insert(1, 10);
    m.insert(2, 20);
    assertEquals(10, m.get(1));
    assertEquals(20, m.get(2));
    assertNull(m.get(3));
  }

  @Test
  void insertOverwrites() {
    HashMapCustom<Integer, Integer> m = new HashMapCustom<>(8);
    m.insert(1, 10);
    m.insert(1, 20);
    assertEquals(20, m.get(1));
  }
}
