package com.polyglot;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class MinHeapTest {
    @Test
    void newIsEmpty() {
        MinHeap h = new MinHeap(8);
        assertEquals(0, h.size());
        assertThrows(IllegalStateException.class, () -> h.peek());
        assertThrows(IllegalStateException.class, () -> h.pop());
    }

    @Test
    void insertAndPeek() {
        MinHeap h = new MinHeap(8);
        h.insert(3);
        h.insert(1);
        h.insert(2);
        assertEquals(3, h.size());
        assertEquals(1, h.peek());
    }

    @Test
    void popReturnsMin() {
        MinHeap h = new MinHeap(8);
        h.insert(3);
        h.insert(1);
        h.insert(2);
        assertEquals(1, h.pop());
        assertEquals(2, h.pop());
        assertEquals(3, h.pop());
        assertThrows(IllegalStateException.class, () -> h.pop());
    }
}
