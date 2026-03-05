package com.polyglot;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class DynamicArrayTest {
  @Test
  void newHasZeroSize() {
    DynamicArray a = new DynamicArray(10);
    assertEquals(0, a.size());
    assertEquals(10, a.capacity());
  }

  @Test
  void pushAndGet() {
    DynamicArray a = new DynamicArray(2);
    a.push(10);
    a.push(20);
    assertEquals(2, a.size());
    assertEquals(10, a.get(0));
    assertEquals(20, a.get(1));
    assertThrows(IndexOutOfBoundsException.class, () -> a.get(2));
  }
}
